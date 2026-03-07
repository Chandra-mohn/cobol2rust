//! CALL/CANCEL runtime support.
//!
//! Provides the `CallableProgram` trait for sub-programs, `CallDispatcher`
//! for dynamic program dispatch, and parameter-passing helpers used by
//! transpiler-generated code.

use std::collections::HashMap;

use cobol_core::error::CallError;
use cobol_core::traits::CobolField;
use cobol_core::config::RuntimeConfig;

// ---------------------------------------------------------------------------
// Parameter passing
// ---------------------------------------------------------------------------

/// How a parameter is passed to a CALLed program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallParamMode {
    ByReference,
    ByContent,
    ByValue,
    Omitted,
}

/// A single parameter in a CALL USING list.
///
/// For BY REFERENCE / BY CONTENT the `field` is populated.
/// For BY VALUE the `value` is populated (numeric only).
/// For OMITTED both are `None`.
pub struct CallParam<'a> {
    pub mode: CallParamMode,
    pub field: Option<&'a mut dyn CobolField>,
    pub value: Option<i64>,
}

impl std::fmt::Debug for CallParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallParam")
            .field("mode", &self.mode)
            .field("has_field", &self.field.is_some())
            .field("value", &self.value)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// CallableProgram trait
// ---------------------------------------------------------------------------

/// Trait implemented by COBOL sub-programs that can be CALLed.
pub trait CallableProgram: std::fmt::Debug {
    /// The PROGRAM-ID of this sub-program.
    fn program_id(&self) -> &str;

    /// Whether this program has the INITIAL attribute (reset state before each call).
    fn is_initial(&self) -> bool {
        false
    }

    /// Execute the program with the given parameters.
    /// Returns RETURN-CODE (0 = success).
    fn call(
        &mut self,
        params: &mut [CallParam],
        config: &RuntimeConfig,
    ) -> Result<i32, CallError>;

    /// Reset the program to its initial state (CANCEL semantics).
    fn cancel(&mut self);
}

// ---------------------------------------------------------------------------
// CallDispatcher
// ---------------------------------------------------------------------------

/// Registry and dispatcher for callable sub-programs.
///
/// Programs are registered by name (case-insensitive) and dispatched
/// when the caller executes a CALL statement.
#[derive(Debug, Default)]
pub struct CallDispatcher {
    programs: HashMap<String, Box<dyn CallableProgram>>,
}

impl CallDispatcher {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    /// Register a callable program. The name is normalized to uppercase.
    pub fn register(&mut self, program: Box<dyn CallableProgram>) {
        let name = program.program_id().to_uppercase();
        self.programs.insert(name, program);
    }

    /// Dispatch a CALL to the named program.
    ///
    /// If the program has the INITIAL attribute, it is cancelled (reset)
    /// before each call per COBOL semantics.
    pub fn call(
        &mut self,
        name: &str,
        params: &mut [CallParam],
        config: &RuntimeConfig,
    ) -> Result<i32, CallError> {
        let key = name.to_uppercase();
        let prog = self
            .programs
            .get_mut(&key)
            .ok_or_else(|| CallError::ProgramNotFound(name.to_string()))?;

        if prog.is_initial() {
            prog.cancel();
        }

        prog.call(params, config)
    }

    /// CANCEL a program -- reset it to initial state.
    pub fn cancel(&mut self, name: &str) -> Result<(), CallError> {
        let key = name.to_uppercase();
        let prog = self
            .programs
            .get_mut(&key)
            .ok_or_else(|| CallError::ProgramNotFound(name.to_string()))?;
        prog.cancel();
        Ok(())
    }

    /// Check if a program is registered.
    pub fn is_registered(&self, name: &str) -> bool {
        self.programs.contains_key(&name.to_uppercase())
    }
}

// ---------------------------------------------------------------------------
// Convenience functions (used by generated code)
// ---------------------------------------------------------------------------

/// Execute a CALL statement. Used by transpiler-generated code.
pub fn cobol_call(
    dispatcher: &mut CallDispatcher,
    name: &str,
    params: &mut [CallParam],
    config: &RuntimeConfig,
) -> Result<i32, CallError> {
    dispatcher.call(name, params, config)
}

/// Execute a CANCEL statement. Used by transpiler-generated code.
pub fn cobol_cancel(
    dispatcher: &mut CallDispatcher,
    name: &str,
) -> Result<(), CallError> {
    dispatcher.cancel(name)
}

/// Create a BY REFERENCE parameter.
pub fn call_param_by_ref(field: &mut dyn CobolField) -> CallParam<'_> {
    CallParam {
        mode: CallParamMode::ByReference,
        field: Some(field),
        value: None,
    }
}

/// Create a BY CONTENT parameter (caller passes a temporary copy).
pub fn call_param_by_content(field: &mut dyn CobolField) -> CallParam<'_> {
    CallParam {
        mode: CallParamMode::ByContent,
        field: Some(field),
        value: None,
    }
}

/// Create a BY VALUE parameter (numeric value only).
pub fn call_param_by_value(val: i64) -> CallParam<'static> {
    CallParam {
        mode: CallParamMode::ByValue,
        field: None,
        value: Some(val),
    }
}

/// Create an OMITTED parameter placeholder.
pub fn call_param_omitted() -> CallParam<'static> {
    CallParam {
        mode: CallParamMode::Omitted,
        field: None,
        value: None,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_core::config::RuntimeConfig;

    /// Mock program for testing.
    #[derive(Debug)]
    struct MockProgram {
        id: String,
        initial: bool,
        call_count: u32,
        cancelled: bool,
    }

    impl MockProgram {
        fn new(id: &str, initial: bool) -> Self {
            Self {
                id: id.to_string(),
                initial,
                call_count: 0,
                cancelled: false,
            }
        }
    }

    impl CallableProgram for MockProgram {
        fn program_id(&self) -> &str {
            &self.id
        }

        fn is_initial(&self) -> bool {
            self.initial
        }

        fn call(
            &mut self,
            _params: &mut [CallParam],
            _config: &RuntimeConfig,
        ) -> Result<i32, CallError> {
            self.call_count += 1;
            self.cancelled = false;
            Ok(i32::try_from(self.call_count).unwrap_or(i32::MAX))
        }

        fn cancel(&mut self) {
            self.cancelled = true;
            self.call_count = 0;
        }
    }

    #[test]
    fn test_dispatcher_register_and_call() {
        let mut dispatcher = CallDispatcher::new();
        let config = RuntimeConfig::default();
        dispatcher.register(Box::new(MockProgram::new("SUBPROG", false)));

        let mut params = [];
        let rc = dispatcher.call("SUBPROG", &mut params, &config).unwrap();
        assert_eq!(rc, 1);

        let rc = dispatcher.call("SUBPROG", &mut params, &config).unwrap();
        assert_eq!(rc, 2);
    }

    #[test]
    fn test_dispatcher_program_not_found() {
        let mut dispatcher = CallDispatcher::new();
        let config = RuntimeConfig::default();
        let mut params = [];
        let result = dispatcher.call("NOSUCH", &mut params, &config);
        assert!(result.is_err());
        match result.unwrap_err() {
            CallError::ProgramNotFound(name) => assert_eq!(name, "NOSUCH"),
            other @ CallError::CallFailed(_) => panic!("expected ProgramNotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_dispatcher_cancel() {
        let mut dispatcher = CallDispatcher::new();
        let config = RuntimeConfig::default();
        dispatcher.register(Box::new(MockProgram::new("SUB1", false)));

        let mut params = [];
        // Call once to increment counter
        let _ = dispatcher.call("SUB1", &mut params, &config).unwrap();

        // Cancel resets
        dispatcher.cancel("SUB1").unwrap();

        // Next call should start fresh (call_count was reset to 0, now becomes 1)
        let rc = dispatcher.call("SUB1", &mut params, &config).unwrap();
        assert_eq!(rc, 1);
    }

    #[test]
    fn test_dispatcher_cancel_not_found() {
        let mut dispatcher = CallDispatcher::new();
        let result = dispatcher.cancel("NOSUCH");
        assert!(result.is_err());
        match result.unwrap_err() {
            CallError::ProgramNotFound(name) => assert_eq!(name, "NOSUCH"),
            other @ CallError::CallFailed(_) => panic!("expected ProgramNotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_dispatcher_initial_program() {
        let mut dispatcher = CallDispatcher::new();
        let config = RuntimeConfig::default();
        dispatcher.register(Box::new(MockProgram::new("INITPROG", true)));

        let mut params = [];
        // First call: cancel resets to 0, then call increments to 1
        let rc = dispatcher.call("INITPROG", &mut params, &config).unwrap();
        assert_eq!(rc, 1);

        // Second call: cancel resets to 0 again, then call increments to 1
        let rc = dispatcher.call("INITPROG", &mut params, &config).unwrap();
        assert_eq!(rc, 1, "INITIAL program should reset before each call");
    }

    #[test]
    fn test_call_param_by_ref() {
        use cobol_types::PackedDecimal;
        use rust_decimal_macros::dec;

        let mut field = PackedDecimal::new(5, 0, false);
        field.pack(dec!(42));
        let param = call_param_by_ref(&mut field);
        assert_eq!(param.mode, CallParamMode::ByReference);
        assert!(param.field.is_some());
        assert!(param.value.is_none());
    }

    #[test]
    fn test_call_param_by_value() {
        let param = call_param_by_value(999);
        assert_eq!(param.mode, CallParamMode::ByValue);
        assert!(param.field.is_none());
        assert_eq!(param.value, Some(999));
    }

    #[test]
    fn test_call_param_omitted() {
        let param = call_param_omitted();
        assert_eq!(param.mode, CallParamMode::Omitted);
        assert!(param.field.is_none());
        assert!(param.value.is_none());
    }
}
