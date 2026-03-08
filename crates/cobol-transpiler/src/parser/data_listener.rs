//! DATA DIVISION listener -- extracts flat data items from the ANTLR4 parse tree.
//!
//! Fires on `dataDescriptionEntryFormat1` (levels 01-49, 77) and
//! `dataDescriptionEntryFormat3` (level 88 conditions). Collects flat
//! items that are later assembled into a hierarchy by [`super::hierarchy`].

#![allow(clippy::wildcard_imports)] // Generated ANTLR4 code has enormous trait lists

use antlr_rust::tree::{ParseTree, ParseTreeListener};

use crate::ast::{
    ConditionValue, DataEntry, Literal, SignClause, SignPosition, Usage,
};
use crate::generated::cobol85listener::Cobol85Listener;
#[allow(clippy::wildcard_imports)]
use crate::generated::cobol85parser::*;
use crate::parser::pic_parser::{compute_storage_size, parse_pic};

/// Listener that fires on DATA DIVISION entries and collects flat `DataEntry` items.
///
/// After the tree walk, call [`super::hierarchy::build_hierarchy`] on `self.items`
/// to build the nested record tree.
#[derive(Debug, Default)]
pub(crate) struct DataDivisionListener {
    pub items: Vec<DataEntry>,
}

impl DataDivisionListener {
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::elidable_lifetime_names)] // Required by ANTLR4 trait signatures
impl<'input> ParseTreeListener<'input, Cobol85ParserContextType> for DataDivisionListener {}

#[allow(clippy::elidable_lifetime_names)] // Required by ANTLR4 trait signatures
impl<'input> Cobol85Listener<'input> for DataDivisionListener {
    fn enter_dataDescriptionEntryFormat1(
        &mut self,
        ctx: &DataDescriptionEntryFormat1Context<'input>,
    ) {
        // Level number: INTEGERLITERAL (01-49) or LEVEL_NUMBER_77
        let level = if let Some(lit) = ctx.INTEGERLITERAL() {
            lit.get_text().trim().parse::<u8>().unwrap_or(0)
        } else if ctx.LEVEL_NUMBER_77().is_some() {
            77
        } else {
            return;
        };

        // Field name: dataName() or FILLER
        let name = if let Some(dn) = ctx.dataName() {
            dn.get_text().trim().to_uppercase()
        } else {
            "FILLER".to_string()
        };

        // PIC clause
        let pic = if let Some(pic_clause) = ctx.dataPictureClause(0) {
            if let Some(pic_str) = pic_clause.pictureString() {
                let raw = pic_str.get_text().trim().to_uppercase();
                // Remove "IS" prefix if present
                let clean = raw.strip_prefix("IS").unwrap_or(&raw).trim();
                if clean.is_empty() {
                    None
                } else {
                    parse_pic(clean).ok()
                }
            } else {
                None
            }
        } else {
            None
        };

        // USAGE clause
        let usage = if let Some(usage_clause) = ctx.dataUsageClause(0) {
            extract_usage(&usage_clause)
        } else {
            Usage::Display
        };

        // VALUE clause
        let value = if let Some(val_clause) = ctx.dataValueClause(0) {
            extract_value_literal(&val_clause)
        } else {
            None
        };

        // REDEFINES clause
        let redefines = ctx
            .dataRedefinesClause(0)
            .and_then(|redef| redef.dataName())
            .map(|dn| dn.get_text().trim().to_uppercase());

        // OCCURS clause
        let (occurs, occurs_depending, index_names) = if let Some(occ) = ctx.dataOccursClause(0) {
            let min_count = if let Some(int_lit) = occ.integerLiteral() {
                int_lit.get_text().trim().parse::<u32>().ok()
            } else {
                None
            };
            // For OCCURS n TO m DEPENDING ON, extract the max (m) from dataOccursTo
            let max_count = occ.dataOccursTo()
                .and_then(|to| to.integerLiteral())
                .and_then(|lit| lit.get_text().trim().parse::<u32>().ok());
            // Use max if present (DEPENDING ON), otherwise use the single count
            let count = max_count.or(min_count);
            let depending = if occ.DEPENDING().is_some() {
                occ.qualifiedDataName()
                    .map(|qdn| qdn.get_text().trim().to_uppercase())
            } else {
                None
            };
            // Extract INDEXED BY names
            let idx_names: Vec<String> = occ.indexName_all()
                .iter()
                .map(|idx| idx.get_text().trim().to_uppercase())
                .collect();
            (count, depending, idx_names)
        } else {
            (None, None, Vec::new())
        };

        // SIGN clause
        let sign = if let Some(sign_clause) = ctx.dataSignClause(0) {
            let text = sign_clause.get_text().to_uppercase();
            let position = if text.contains("LEADING") {
                SignPosition::Leading
            } else {
                SignPosition::Trailing
            };
            let separate = text.contains("SEPARATE");
            Some(SignClause { position, separate })
        } else {
            None
        };

        // JUSTIFIED clause
        let justified_right = ctx.dataJustifiedClause(0).is_some();

        // BLANK WHEN ZERO clause
        let blank_when_zero = ctx.dataBlankWhenZeroClause(0).is_some();

        // Compute byte length from PIC + USAGE
        let byte_length = pic.as_ref().map(|p| compute_storage_size(p, usage) as usize);

        self.items.push(DataEntry {
            level,
            name,
            pic,
            usage,
            value,
            redefines,
            occurs,
            occurs_depending,
            sign,
            justified_right,
            blank_when_zero,
            children: Vec::new(),
            condition_values: Vec::new(),
            byte_offset: None,
            byte_length,
            renames_target: None,
            renames_thru: None,
            index_names,
        });
    }

    fn enter_dataDescriptionEntryFormat3(
        &mut self,
        ctx: &DataDescriptionEntryFormat3Context<'input>,
    ) {
        // 88-level condition name
        let name = if let Some(cn) = ctx.conditionName() {
            cn.get_text().trim().to_uppercase()
        } else {
            return;
        };

        // Extract condition values
        let mut condition_values = Vec::new();
        if let Some(val_clause) = ctx.dataValueClause() {
            for interval in val_clause.dataValueInterval_all() {
                let text = interval.get_text().trim().to_string();
                let clean = strip_cobol_quotes(&text);
                if !clean.is_empty() {
                    // Check for THRU/THROUGH range
                    let upper = clean.to_uppercase();
                    if upper.contains("THRU") || upper.contains("THROUGH") {
                        // Try to split on THRU/THROUGH
                        let parts: Vec<&str> = if upper.contains("THRU") {
                            clean.splitn(2, "THRU").collect()
                        } else {
                            clean.splitn(2, "THROUGH").collect()
                        };
                        if parts.len() == 2 {
                            let low = strip_cobol_quotes(parts[0].trim());
                            let high = strip_cobol_quotes(parts[1].trim());
                            condition_values.push(ConditionValue::Range {
                                low: parse_condition_literal(&low),
                                high: parse_condition_literal(&high),
                            });
                        } else {
                            condition_values
                                .push(ConditionValue::Single(parse_condition_literal(&clean)));
                        }
                    } else {
                        condition_values
                            .push(ConditionValue::Single(parse_condition_literal(&clean)));
                    }
                }
            }
        }

        self.items.push(DataEntry {
            level: 88,
            name,
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: Vec::new(),
            condition_values,
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: Vec::new(),
        });
    }

    fn enter_dataDescriptionEntryFormat2(
        &mut self,
        ctx: &DataDescriptionEntryFormat2Context<'input>,
    ) {
        // 66-level RENAMES clause
        let name = if let Some(dn) = ctx.dataName() {
            dn.get_text().trim().to_uppercase()
        } else {
            return;
        };

        let (renames_target, renames_thru) = if let Some(rc) = ctx.dataRenamesClause() {
            let qdn_all = rc.qualifiedDataName_all();
            let target = qdn_all
                .first()
                .map(|qdn| qdn.get_text().trim().to_uppercase());
            let thru = if rc.THROUGH().is_some() || rc.THRU().is_some() {
                qdn_all
                    .get(1)
                    .map(|qdn| qdn.get_text().trim().to_uppercase())
            } else {
                None
            };
            (target, thru)
        } else {
            (None, None)
        };

        self.items.push(DataEntry {
            level: 66,
            name,
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: Vec::new(),
            condition_values: Vec::new(),
            byte_offset: None,
            byte_length: None,
            renames_target,
            renames_thru,
            index_names: Vec::new(),
        });
    }
}

/// Extract USAGE enum from a `DataUsageClause` context.
fn extract_usage(ctx: &DataUsageClauseContext<'_>) -> Usage {
    if ctx.COMP_3().is_some() || ctx.COMPUTATIONAL_3().is_some() || ctx.PACKED_DECIMAL().is_some() {
        Usage::Comp3
    } else if ctx.COMP_1().is_some() || ctx.COMPUTATIONAL_1().is_some() {
        Usage::Comp1
    } else if ctx.COMP_2().is_some() || ctx.COMPUTATIONAL_2().is_some() {
        Usage::Comp2
    } else if ctx.COMP_5().is_some() || ctx.COMPUTATIONAL_5().is_some() {
        Usage::Comp5
    } else if ctx.COMP().is_some()
        || ctx.COMPUTATIONAL().is_some()
        || ctx.COMP_4().is_some()
        || ctx.COMPUTATIONAL_4().is_some()
        || ctx.BINARY().is_some()
    {
        Usage::Comp
    } else if ctx.INDEX().is_some() {
        Usage::Index
    } else if ctx.POINTER().is_some() {
        Usage::Pointer
    } else {
        Usage::Display
    }
}

/// Extract a VALUE literal from a `DataValueClause`.
fn extract_value_literal(ctx: &DataValueClauseContext<'_>) -> Option<Literal> {
    let intervals = ctx.dataValueInterval_all();
    if intervals.is_empty() {
        return None;
    }
    let text = intervals[0].get_text().trim().to_string();
    let clean = strip_cobol_quotes(&text);
    if clean.is_empty() {
        return None;
    }
    Some(parse_condition_literal(&clean))
}

/// Parse a literal string into a `Literal` enum.
fn parse_condition_literal(s: &str) -> Literal {
    let upper = s.trim().to_uppercase();

    // Check for figurative constants
    match upper.as_str() {
        "SPACES" | "SPACE" => return Literal::Figurative(crate::ast::FigurativeConstant::Spaces),
        "ZEROS" | "ZEROES" | "ZERO" => {
            return Literal::Figurative(crate::ast::FigurativeConstant::Zeros);
        }
        "HIGH-VALUES" | "HIGH-VALUE" => {
            return Literal::Figurative(crate::ast::FigurativeConstant::HighValues);
        }
        "LOW-VALUES" | "LOW-VALUE" => {
            return Literal::Figurative(crate::ast::FigurativeConstant::LowValues);
        }
        "QUOTES" | "QUOTE" => {
            return Literal::Figurative(crate::ast::FigurativeConstant::Quotes);
        }
        "NULLS" | "NULL" => {
            return Literal::Figurative(crate::ast::FigurativeConstant::Nulls);
        }
        _ => {}
    }

    // Check for ALL "x" pattern (ANTLR concatenates to ALL"x")
    if upper.starts_with("ALL") {
        let rest = &s.trim()[3..];
        let inner = strip_cobol_quotes(rest);
        if !inner.is_empty() {
            return Literal::Figurative(crate::ast::FigurativeConstant::All(inner));
        }
    }

    // Check if numeric (digits, optional sign, optional decimal point)
    let trimmed = s.trim();
    if trimmed
        .bytes()
        .all(|b| b.is_ascii_digit() || b == b'+' || b == b'-' || b == b'.')
        && !trimmed.is_empty()
    {
        Literal::Numeric(trimmed.to_string())
    } else {
        Literal::Alphanumeric(trimmed.to_string())
    }
}

/// Strip single or double quotes from a COBOL literal value.
fn strip_cobol_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        if trimmed.len() >= 2 {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            String::new()
        }
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_quotes_single() {
        assert_eq!(strip_cobol_quotes("'A'"), "A");
    }

    #[test]
    fn strip_quotes_double() {
        assert_eq!(strip_cobol_quotes("\"CR\""), "CR");
    }

    #[test]
    fn strip_quotes_none() {
        assert_eq!(strip_cobol_quotes("42"), "42");
    }

    #[test]
    fn parse_literal_numeric() {
        match parse_condition_literal("42") {
            Literal::Numeric(s) => assert_eq!(s, "42"),
            other => panic!("expected Numeric, got {other:?}"),
        }
    }

    #[test]
    fn parse_literal_spaces() {
        match parse_condition_literal("SPACES") {
            Literal::Figurative(crate::ast::FigurativeConstant::Spaces) => {}
            other => panic!("expected Figurative(Spaces), got {other:?}"),
        }
    }

    #[test]
    fn parse_literal_alphanumeric() {
        match parse_condition_literal("HELLO") {
            Literal::Alphanumeric(s) => assert_eq!(s, "HELLO"),
            other => panic!("expected Alphanumeric, got {other:?}"),
        }
    }
}
