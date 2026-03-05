use cobol_core::config::RuntimeConfig;

use crate::perform_stack::PerformStack;

/// Execution context for a COBOL program.
///
/// Holds runtime configuration, special registers, and the perform stack.
/// One `CobolProgram` instance exists per program execution.
#[derive(Debug)]
pub struct CobolProgram {
    /// Runtime configuration (dialect, rounding, diagnostics, etc.)
    pub config: RuntimeConfig,
    /// RETURN-CODE special register
    pub return_code: i32,
    /// SORT-RETURN special register (0=success, 16=failed)
    pub sort_return: i32,
    /// TALLY special register (used by INSPECT)
    pub tally: i32,
    /// Perform stack for tracking PERFORM ranges
    pub perform_stack: PerformStack,
}

impl CobolProgram {
    /// Create a new program context with default configuration.
    pub fn new() -> Self {
        Self {
            config: RuntimeConfig::default(),
            return_code: 0,
            sort_return: 0,
            tally: 0,
            perform_stack: PerformStack::new(),
        }
    }

    /// Create a new program context with the given configuration.
    pub fn with_config(config: RuntimeConfig) -> Self {
        Self {
            config,
            return_code: 0,
            sort_return: 0,
            tally: 0,
            perform_stack: PerformStack::new(),
        }
    }

    /// STOP RUN: terminate the program, returning the return code.
    pub fn stop_run(&self) -> i32 {
        self.return_code
    }

    /// GOBACK: same as STOP RUN for a main program.
    pub fn goback(&self) -> i32 {
        self.return_code
    }
}

impl Default for CobolProgram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_program_defaults() {
        let prog = CobolProgram::new();
        assert_eq!(prog.return_code, 0);
        assert_eq!(prog.sort_return, 0);
        assert_eq!(prog.tally, 0);
    }

    #[test]
    fn stop_run_returns_code() {
        let mut prog = CobolProgram::new();
        prog.return_code = 42;
        assert_eq!(prog.stop_run(), 42);
    }
}
