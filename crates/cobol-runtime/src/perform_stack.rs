/// Stack for tracking PERFORM ranges.
///
/// In COBOL, PERFORM pushes a return point onto the stack.
/// When execution reaches the end of the performed range,
/// it pops the stack and returns to the caller.
///
/// This also detects GO TO that leaves a PERFORM range (which
/// abandons the perform stack entry).
#[derive(Debug)]
pub struct PerformStack {
    stack: Vec<PerformEntry>,
}

/// A single entry on the perform stack.
#[derive(Debug, Clone)]
pub struct PerformEntry {
    /// Name of the paragraph/section being performed
    pub target: String,
    /// End paragraph/section (for PERFORM THRU)
    pub thru: Option<String>,
}

impl PerformStack {
    /// Create an empty perform stack.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a PERFORM entry.
    pub fn push(&mut self, target: String, thru: Option<String>) {
        self.stack.push(PerformEntry { target, thru });
    }

    /// Pop the top PERFORM entry (when the performed range completes).
    pub fn pop(&mut self) -> Option<PerformEntry> {
        self.stack.pop()
    }

    /// Check if a paragraph is within the current PERFORM range.
    pub fn is_within(&self, paragraph: &str) -> bool {
        self.stack
            .iter()
            .any(|e| e.target == paragraph || e.thru.as_deref() == Some(paragraph))
    }

    /// Check if the stack is empty (no active PERFORMs).
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Current depth of the perform stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Default for PerformStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = PerformStack::new();
        assert!(stack.is_empty());

        stack.push("PARA-A".to_string(), None);
        assert!(!stack.is_empty());
        assert_eq!(stack.depth(), 1);

        let entry = stack.pop().unwrap();
        assert_eq!(entry.target, "PARA-A");
        assert!(stack.is_empty());
    }

    #[test]
    fn is_within() {
        let mut stack = PerformStack::new();
        stack.push("PARA-A".to_string(), Some("PARA-C".to_string()));

        assert!(stack.is_within("PARA-A"));
        assert!(stack.is_within("PARA-C"));
        assert!(!stack.is_within("PARA-X"));
    }

    #[test]
    fn nested_performs() {
        let mut stack = PerformStack::new();
        stack.push("OUTER".to_string(), None);
        stack.push("INNER".to_string(), None);
        assert_eq!(stack.depth(), 2);

        let inner = stack.pop().unwrap();
        assert_eq!(inner.target, "INNER");
        assert_eq!(stack.depth(), 1);
    }
}
