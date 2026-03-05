use rust_decimal::Decimal;

/// A single condition value for a Level 88 item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level88Value {
    /// Single value: VALUE "A" or VALUE 100
    Single(ConditionValue),
    /// Range: VALUE 1 THRU 10
    Range(ConditionValue, ConditionValue),
}

/// A condition value can be either alphanumeric or numeric.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionValue {
    Alphanumeric(Vec<u8>),
    Numeric(Decimal),
}

/// Level 88 condition predicate.
///
/// COBOL Level 88 items define named conditions on a parent data item.
/// They don't have storage of their own -- they test the parent's value.
///
/// ```cobol
///     05 WS-STATUS  PIC X(2).
///        88 STATUS-ACTIVE  VALUE "AC".
///        88 STATUS-PENDING VALUE "PE" "PR".
///        88 STATUS-VALID   VALUE "AC" THRU "ZZ".
/// ```
#[derive(Debug, Clone)]
pub struct Level88Predicate {
    /// Name of the condition (e.g., "STATUS-ACTIVE")
    pub name: String,
    /// Accepted values/ranges
    pub values: Vec<Level88Value>,
}

impl Level88Predicate {
    /// Create a predicate with a single value.
    pub fn single(name: &str, value: ConditionValue) -> Self {
        Self {
            name: name.to_string(),
            values: vec![Level88Value::Single(value)],
        }
    }

    /// Create a predicate with a range (THRU).
    pub fn range(name: &str, low: ConditionValue, high: ConditionValue) -> Self {
        Self {
            name: name.to_string(),
            values: vec![Level88Value::Range(low, high)],
        }
    }

    /// Create a predicate with multiple values.
    pub fn multi(name: &str, values: Vec<Level88Value>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }

    /// Test whether a numeric value matches this condition.
    pub fn matches_numeric(&self, value: Decimal) -> bool {
        self.values.iter().any(|v| match v {
            Level88Value::Single(ConditionValue::Numeric(n)) => value == *n,
            Level88Value::Range(ConditionValue::Numeric(low), ConditionValue::Numeric(high)) => {
                value >= *low && value <= *high
            }
            _ => false,
        })
    }

    /// Test whether an alphanumeric value matches this condition.
    pub fn matches_alphanumeric(&self, value: &[u8]) -> bool {
        self.values.iter().any(|v| match v {
            Level88Value::Single(ConditionValue::Alphanumeric(s)) => value == s.as_slice(),
            Level88Value::Range(
                ConditionValue::Alphanumeric(low),
                ConditionValue::Alphanumeric(high),
            ) => value >= low.as_slice() && value <= high.as_slice(),
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn single_numeric_match() {
        let pred = Level88Predicate::single(
            "IS-HUNDRED",
            ConditionValue::Numeric(dec!(100)),
        );
        assert!(pred.matches_numeric(dec!(100)));
        assert!(!pred.matches_numeric(dec!(99)));
    }

    #[test]
    fn range_numeric_match() {
        let pred = Level88Predicate::range(
            "VALID-RANGE",
            ConditionValue::Numeric(dec!(1)),
            ConditionValue::Numeric(dec!(10)),
        );
        assert!(pred.matches_numeric(dec!(1)));
        assert!(pred.matches_numeric(dec!(5)));
        assert!(pred.matches_numeric(dec!(10)));
        assert!(!pred.matches_numeric(dec!(0)));
        assert!(!pred.matches_numeric(dec!(11)));
    }

    #[test]
    fn single_alphanumeric_match() {
        let pred = Level88Predicate::single(
            "STATUS-ACTIVE",
            ConditionValue::Alphanumeric(b"AC".to_vec()),
        );
        assert!(pred.matches_alphanumeric(b"AC"));
        assert!(!pred.matches_alphanumeric(b"PE"));
    }

    #[test]
    fn range_alphanumeric_match() {
        let pred = Level88Predicate::range(
            "VALID-CODES",
            ConditionValue::Alphanumeric(b"AA".to_vec()),
            ConditionValue::Alphanumeric(b"ZZ".to_vec()),
        );
        assert!(pred.matches_alphanumeric(b"AC"));
        assert!(pred.matches_alphanumeric(b"MM"));
        assert!(!pred.matches_alphanumeric(b"  "));
    }

    #[test]
    fn multi_value_match() {
        let pred = Level88Predicate::multi(
            "STATUS-PENDING",
            vec![
                Level88Value::Single(ConditionValue::Alphanumeric(b"PE".to_vec())),
                Level88Value::Single(ConditionValue::Alphanumeric(b"PR".to_vec())),
            ],
        );
        assert!(pred.matches_alphanumeric(b"PE"));
        assert!(pred.matches_alphanumeric(b"PR"));
        assert!(!pred.matches_alphanumeric(b"AC"));
    }

    #[test]
    fn mixed_multi_value() {
        let pred = Level88Predicate::multi(
            "COMPLEX",
            vec![
                Level88Value::Single(ConditionValue::Numeric(dec!(0))),
                Level88Value::Range(ConditionValue::Numeric(dec!(10)), ConditionValue::Numeric(dec!(20))),
            ],
        );
        assert!(pred.matches_numeric(dec!(0)));
        assert!(pred.matches_numeric(dec!(15)));
        assert!(!pred.matches_numeric(dec!(5)));
    }
}
