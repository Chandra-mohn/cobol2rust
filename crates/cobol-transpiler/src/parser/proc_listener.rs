//! PROCEDURE DIVISION listener -- extracts procedure structure from the ANTLR4 parse tree.
//!
//! Uses a hybrid approach:
//! - Listener callbacks track section/paragraph boundaries
//! - Recursive extraction functions walk the context tree for nested statements
//! - This naturally handles IF/EVALUATE/PERFORM nesting without complex stack management

#![allow(clippy::wildcard_imports)] // Generated ANTLR4 code has enormous trait lists

use antlr_rust::tree::{ParseTree, ParseTreeListener};

use crate::ast::*;
use crate::generated::cobol85listener::Cobol85Listener;
#[allow(clippy::wildcard_imports)]
use crate::generated::cobol85parser::*;

/// Listener that fires on PROCEDURE DIVISION entries and collects structure.
///
/// After the tree walk, `sections` and `paragraphs` contain the full
/// procedure division AST.
#[derive(Debug, Default)]
pub(crate) struct ProcedureDivisionListener {
    /// Sections with their paragraphs.
    pub sections: Vec<Section>,
    /// Paragraphs not inside any section.
    pub paragraphs: Vec<Paragraph>,
    /// Whether we're inside the procedure division.
    in_proc_div: bool,
    /// Current section name (if inside a section).
    current_section_name: Option<String>,
    /// Paragraphs collected for the current section.
    section_paragraphs: Vec<Paragraph>,
}

impl ProcedureDivisionListener {
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'input> ParseTreeListener<'input, Cobol85ParserContextType> for ProcedureDivisionListener {}

#[allow(clippy::elidable_lifetime_names)]
impl<'input> Cobol85Listener<'input> for ProcedureDivisionListener {
    fn enter_procedureDivision(
        &mut self,
        _ctx: &ProcedureDivisionContext<'input>,
    ) {
        self.in_proc_div = true;
    }

    fn exit_procedureDivision(
        &mut self,
        _ctx: &ProcedureDivisionContext<'input>,
    ) {
        self.in_proc_div = false;
        // Flush remaining section if any
        if let Some(name) = self.current_section_name.take() {
            self.sections.push(Section {
                name,
                paragraphs: std::mem::take(&mut self.section_paragraphs),
            });
        }
    }

    fn enter_procedureSection(
        &mut self,
        ctx: &ProcedureSectionContext<'input>,
    ) {
        if !self.in_proc_div {
            return;
        }
        // Flush previous section if any
        if let Some(name) = self.current_section_name.take() {
            self.sections.push(Section {
                name,
                paragraphs: std::mem::take(&mut self.section_paragraphs),
            });
        }
        // Start new section
        self.current_section_name = ctx
            .procedureSectionHeader()
            .and_then(|h| h.sectionName())
            .map(|sn| sn.get_text().trim().to_uppercase());
    }

    fn exit_paragraph(
        &mut self,
        ctx: &ParagraphContext<'input>,
    ) {
        if !self.in_proc_div {
            return;
        }

        let name = ctx
            .paragraphName()
            .map(|pn| pn.get_text().trim().to_uppercase())
            .unwrap_or_default();

        // Extract all statements from all sentences in this paragraph
        let mut sentences = Vec::new();
        for sentence_ctx in ctx.sentence_all() {
            let mut statements = Vec::new();
            for stmt_ctx in sentence_ctx.statement_all() {
                if let Some(stmt) = extract_statement(&stmt_ctx) {
                    statements.push(stmt);
                }
            }
            if !statements.is_empty() {
                sentences.push(Sentence { statements });
            }
        }

        let para = Paragraph { name, sentences };

        if self.current_section_name.is_some() {
            self.section_paragraphs.push(para);
        } else {
            self.paragraphs.push(para);
        }
    }
}

// ---------------------------------------------------------------------------
// Statement extraction (recursive)
// ---------------------------------------------------------------------------

/// Extract a Statement from a `StatementContext` by checking which child rule matched.
fn extract_statement(ctx: &StatementContext<'_>) -> Option<Statement> {
    if let Some(c) = ctx.moveStatement() {
        return Some(extract_move(&c));
    }
    if let Some(c) = ctx.displayStatement() {
        return Some(extract_display(&c));
    }
    if let Some(c) = ctx.addStatement() {
        return Some(extract_add(&c));
    }
    if let Some(c) = ctx.subtractStatement() {
        return Some(extract_subtract(&c));
    }
    if let Some(c) = ctx.multiplyStatement() {
        return Some(extract_multiply(&c));
    }
    if let Some(c) = ctx.divideStatement() {
        return Some(extract_divide(&c));
    }
    if let Some(c) = ctx.computeStatement() {
        return Some(extract_compute(&c));
    }
    if let Some(c) = ctx.ifStatement() {
        return Some(extract_if(&c));
    }
    if let Some(c) = ctx.evaluateStatement() {
        return Some(extract_evaluate(&c));
    }
    if let Some(c) = ctx.performStatement() {
        return Some(extract_perform(&c));
    }
    if let Some(c) = ctx.goToStatement() {
        return Some(extract_goto(&c));
    }
    if let Some(c) = ctx.stopStatement() {
        return Some(extract_stop(&c));
    }
    if ctx.gobackStatement().is_some() {
        return Some(Statement::GoBack);
    }
    if let Some(c) = ctx.initializeStatement() {
        return Some(extract_initialize(&c));
    }
    if ctx.continueStatement().is_some() {
        return Some(Statement::Continue);
    }
    if let Some(c) = ctx.callStatement() {
        return Some(extract_call(&c));
    }
    if let Some(c) = ctx.cancelStatement() {
        return Some(extract_cancel(&c));
    }
    if let Some(c) = ctx.acceptStatement() {
        return Some(extract_accept(&c));
    }
    if let Some(c) = ctx.exitStatement() {
        let text = c.get_text().to_uppercase();
        if text.contains("PROGRAM") {
            return Some(Statement::ExitProgram);
        } else if text.contains("SECTION") {
            return Some(Statement::ExitSection);
        } else if text.contains("PARAGRAPH") {
            return Some(Statement::ExitParagraph);
        }
        // Plain EXIT -> same as EXIT PARAGRAPH (COBOL standard)
        return Some(Statement::ExitParagraph);
    }
    if let Some(c) = ctx.setStatement() {
        return Some(extract_set(&c));
    }
    if let Some(c) = ctx.startStatement() {
        return Some(extract_start(&c));
    }
    if let Some(c) = ctx.openStatement() {
        return Some(extract_open(&c));
    }
    if let Some(c) = ctx.closeStatement() {
        return Some(extract_close(&c));
    }
    if let Some(c) = ctx.readStatement() {
        return Some(extract_read(&c));
    }
    if let Some(c) = ctx.writeStatement() {
        return Some(extract_write(&c));
    }
    if let Some(c) = ctx.rewriteStatement() {
        return Some(extract_rewrite(&c));
    }
    if let Some(c) = ctx.deleteStatement() {
        return Some(extract_delete(&c));
    }
    if let Some(c) = ctx.sortStatement() {
        return Some(extract_sort(&c));
    }
    if let Some(c) = ctx.mergeStatement() {
        return Some(extract_merge(&c));
    }
    if let Some(c) = ctx.releaseStatement() {
        return Some(extract_release(&c));
    }
    if let Some(c) = ctx.returnStatement() {
        return Some(extract_return(&c));
    }
    if let Some(c) = ctx.inspectStatement() {
        return Some(extract_inspect(&c));
    }
    if let Some(c) = ctx.stringStatement() {
        return Some(extract_string(&c));
    }
    if let Some(c) = ctx.unstringStatement() {
        return Some(extract_unstring(&c));
    }
    // Unsupported statement -- skip
    None
}

// ---------------------------------------------------------------------------
// Individual statement extractors
// ---------------------------------------------------------------------------

fn extract_move(ctx: &MoveStatementContext<'_>) -> Statement {
    if let Some(corr_ctx) = ctx.moveCorrespondingToStatement() {
        let source = corr_ctx
            .moveCorrespondingToSendingArea().map_or_else(|| Operand::Literal(Literal::Alphanumeric(String::new())), |s| extract_identifier_or_literal_from_text(&s.get_text()));
        let destinations: Vec<DataReference> = corr_ctx
            .identifier_all()
            .iter()
            .map(|id| extract_data_ref_from_identifier(id))
            .collect();
        Statement::Move(MoveStatement {
            corresponding: true,
            source,
            destinations,
        })
    } else if let Some(move_to) = ctx.moveToStatement() {
        let source = move_to
            .moveToSendingArea().map_or_else(|| Operand::Literal(Literal::Alphanumeric(String::new())), |s| extract_operand_from_sending_area(&s));
        let destinations: Vec<DataReference> = move_to
            .identifier_all()
            .iter()
            .map(|id| extract_data_ref_from_identifier(id))
            .collect();
        Statement::Move(MoveStatement {
            corresponding: false,
            source,
            destinations,
        })
    } else {
        // Fallback: create a MOVE from raw text
        Statement::Move(MoveStatement {
            corresponding: false,
            source: Operand::Literal(Literal::Alphanumeric(String::new())),
            destinations: Vec::new(),
        })
    }
}

fn extract_display(ctx: &DisplayStatementContext<'_>) -> Statement {
    let items: Vec<Operand> = ctx
        .displayOperand_all()
        .iter()
        .map(|op| {
            if let Some(id) = op.identifier() {
                extract_operand_from_identifier(&id)
            } else if let Some(lit) = op.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Alphanumeric(op.get_text()))
            }
        })
        .collect();

    let no_advancing = ctx
        .displayWith()
        .is_some();

    let upon = ctx.displayUpon().map(|u| {
        let text = u.get_text().to_uppercase();
        if text.contains("SYSERR") || text.contains("STANDARD-ERROR") {
            DisplayTarget::Syserr
        } else if text.contains("CONSOLE") {
            DisplayTarget::Console
        } else {
            DisplayTarget::Sysout
        }
    });

    Statement::Display(DisplayStatement {
        items,
        upon: upon.unwrap_or_default(),
        no_advancing,
    })
}

fn extract_add(ctx: &AddStatementContext<'_>) -> Statement {
    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    if let Some(giving_ctx) = ctx.addToGivingStatement() {
        let operands: Vec<Operand> = giving_ctx
            .addFrom_all()
            .iter()
            .map(|f| extract_operand_from_add_from(f))
            .collect();
        let to: Vec<ArithTarget> = giving_ctx
            .addToGiving_all()
            .iter()
            .map(|t| ArithTarget {
                field: make_data_ref(&t.get_text()),
                rounded: false,
            })
            .collect();
        let giving: Vec<ArithTarget> = giving_ctx
            .addGiving_all()
            .iter()
            .map(|g| ArithTarget {
                field: extract_data_ref_from_identifier_text(&g.get_text()),
                rounded: g.ROUNDED().is_some(),
            })
            .collect();
        Statement::Add(AddStatement {
            operands,
            to,
            giving,
            on_size_error,
            not_on_size_error,
            corresponding: false,
        })
    } else if let Some(to_ctx) = ctx.addToStatement() {
        let operands: Vec<Operand> = to_ctx
            .addFrom_all()
            .iter()
            .map(|f| extract_operand_from_add_from(f))
            .collect();
        let to: Vec<ArithTarget> = to_ctx
            .addTo_all()
            .iter()
            .map(|t| ArithTarget {
                field: t.identifier().map_or_else(
                    || extract_data_ref_from_identifier_text(&t.get_text()),
                    |id| extract_data_ref_from_identifier(&id),
                ),
                rounded: t.ROUNDED().is_some(),
            })
            .collect();
        Statement::Add(AddStatement {
            operands,
            to,
            giving: Vec::new(),
            on_size_error,
            not_on_size_error,
            corresponding: false,
        })
    } else if ctx.addCorrespondingStatement().is_some() {
        let text = ctx.get_text();
        Statement::Add(AddStatement {
            operands: vec![extract_identifier_or_literal_from_text(&text)],
            to: Vec::new(),
            giving: Vec::new(),
            on_size_error,
            not_on_size_error,
            corresponding: true,
        })
    } else {
        Statement::Add(AddStatement {
            operands: Vec::new(),
            to: Vec::new(),
            giving: Vec::new(),
            on_size_error: Vec::new(),
            not_on_size_error: Vec::new(),
            corresponding: false,
        })
    }
}

fn extract_subtract(ctx: &SubtractStatementContext<'_>) -> Statement {
    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    if let Some(giving_ctx) = ctx.subtractFromGivingStatement() {
        let operands: Vec<Operand> = giving_ctx
            .subtractSubtrahend_all()
            .iter()
            .map(|s| extract_operand_from_identifier_or_literal_ctx(&s.get_text()))
            .collect();
        let from_text = giving_ctx
            .subtractMinuendGiving()
            .map(|m| m.get_text())
            .unwrap_or_default();
        let from = vec![ArithTarget {
            field: make_data_ref(&from_text),
            rounded: false,
        }];
        let giving: Vec<ArithTarget> = giving_ctx
            .subtractGiving_all()
            .iter()
            .map(|g| ArithTarget {
                field: extract_data_ref_from_identifier_text(&g.get_text()),
                rounded: g.ROUNDED().is_some(),
            })
            .collect();
        Statement::Subtract(SubtractStatement {
            operands,
            from,
            giving,
            on_size_error,
            not_on_size_error,
            corresponding: false,
        })
    } else if let Some(from_ctx) = ctx.subtractFromStatement() {
        let operands: Vec<Operand> = from_ctx
            .subtractSubtrahend_all()
            .iter()
            .map(|s| extract_operand_from_identifier_or_literal_ctx(&s.get_text()))
            .collect();
        let from: Vec<ArithTarget> = from_ctx
            .subtractMinuend_all()
            .iter()
            .map(|m| ArithTarget {
                field: m.identifier().map_or_else(
                    || extract_data_ref_from_identifier_text(&m.get_text()),
                    |id| extract_data_ref_from_identifier_text(&id.get_text()),
                ),
                rounded: m.ROUNDED().is_some(),
            })
            .collect();
        Statement::Subtract(SubtractStatement {
            operands,
            from,
            giving: Vec::new(),
            on_size_error,
            not_on_size_error,
            corresponding: false,
        })
    } else {
        Statement::Subtract(SubtractStatement {
            operands: Vec::new(),
            from: Vec::new(),
            giving: Vec::new(),
            on_size_error: Vec::new(),
            not_on_size_error: Vec::new(),
            corresponding: false,
        })
    }
}

fn extract_multiply(ctx: &MultiplyStatementContext<'_>) -> Statement {
    let multiplicand = if let Some(id) = ctx.identifier() {
        extract_operand_from_identifier(&id)
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else {
        Operand::Literal(Literal::Numeric("0".to_string()))
    };

    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    if let Some(giving_ctx) = ctx.multiplyGiving() {
        let by_text = giving_ctx
            .multiplyGivingOperand()
            .map(|o| o.get_text())
            .unwrap_or_default();
        let _by_operand = extract_operand_from_identifier_or_literal_ctx(&by_text);
        let giving: Vec<ArithTarget> = giving_ctx
            .multiplyGivingResult_all()
            .iter()
            .map(|r| ArithTarget {
                field: r.identifier().map_or_else(
                    || extract_data_ref_from_identifier_text(&r.get_text()),
                    |id| extract_data_ref_from_identifier_text(&id.get_text()),
                ),
                rounded: r.ROUNDED().is_some(),
            })
            .collect();
        Statement::Multiply(MultiplyStatement {
            operand: multiplicand,
            by: vec![ArithTarget {
                field: make_data_ref(&by_text),
                rounded: false,
            }],
            giving,
            on_size_error,
            not_on_size_error,
        })
    } else if let Some(regular_ctx) = ctx.multiplyRegular() {
        let by: Vec<ArithTarget> = regular_ctx
            .multiplyRegularOperand_all()
            .iter()
            .map(|o| ArithTarget {
                field: o.identifier().map_or_else(
                    || extract_data_ref_from_identifier_text(&o.get_text()),
                    |id| extract_data_ref_from_identifier_text(&id.get_text()),
                ),
                rounded: o.ROUNDED().is_some(),
            })
            .collect();
        Statement::Multiply(MultiplyStatement {
            operand: multiplicand,
            by,
            giving: Vec::new(),
            on_size_error,
            not_on_size_error,
        })
    } else {
        Statement::Multiply(MultiplyStatement {
            operand: multiplicand,
            by: Vec::new(),
            giving: Vec::new(),
            on_size_error: Vec::new(),
            not_on_size_error: Vec::new(),
        })
    }
}

fn extract_divide(ctx: &DivideStatementContext<'_>) -> Statement {
    let operand = if let Some(id) = ctx.identifier() {
        extract_operand_from_identifier(&id)
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else {
        Operand::Literal(Literal::Numeric("0".to_string()))
    };

    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());
    let remainder = ctx.divideRemainder().map(|r| ArithTarget {
        field: r.identifier().map_or_else(
            || extract_data_ref_from_identifier_text(&r.get_text()),
            |id| extract_data_ref_from_identifier_text(&id.get_text()),
        ),
        rounded: false,
    });

    if let Some(into_giving) = ctx.divideIntoGivingStatement() {
        let into_operand = if let Some(id) = into_giving.identifier() {
            extract_operand_from_identifier(&id)
        } else if let Some(lit) = into_giving.literal() {
            extract_literal_operand(&lit)
        } else {
            Operand::Literal(Literal::Numeric("0".to_string()))
        };
        let giving = into_giving
            .divideGivingPhrase()
            .map(|gp| extract_divide_giving_targets(&gp))
            .unwrap_or_default();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::Into,
            into: vec![ArithTarget {
                field: operand_to_data_ref(&into_operand),
                rounded: false,
            }],
            by_operand: None,
            giving,
            remainder,
            on_size_error,
            not_on_size_error,
        })
    } else if let Some(by_giving) = ctx.divideByGivingStatement() {
        let by_op = if let Some(id) = by_giving.identifier() {
            extract_operand_from_identifier(&id)
        } else if let Some(lit) = by_giving.literal() {
            extract_literal_operand(&lit)
        } else {
            Operand::Literal(Literal::Numeric("0".to_string()))
        };
        let giving = by_giving
            .divideGivingPhrase()
            .map(|gp| extract_divide_giving_targets(&gp))
            .unwrap_or_default();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::By,
            into: Vec::new(),
            by_operand: Some(by_op),
            giving,
            remainder,
            on_size_error,
            not_on_size_error,
        })
    } else if let Some(into_ctx) = ctx.divideIntoStatement() {
        let into: Vec<ArithTarget> = into_ctx
            .divideInto_all()
            .iter()
            .map(|d| ArithTarget {
                field: d.identifier().map_or_else(
                    || extract_data_ref_from_identifier_text(&d.get_text()),
                    |id| extract_data_ref_from_identifier_text(&id.get_text()),
                ),
                rounded: d.ROUNDED().is_some(),
            })
            .collect();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::Into,
            into,
            by_operand: None,
            giving: Vec::new(),
            remainder,
            on_size_error,
            not_on_size_error,
        })
    } else {
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::Into,
            into: Vec::new(),
            by_operand: None,
            giving: Vec::new(),
            remainder: None,
            on_size_error: Vec::new(),
            not_on_size_error: Vec::new(),
        })
    }
}

fn extract_compute(ctx: &ComputeStatementContext<'_>) -> Statement {
    let targets: Vec<ArithTarget> = ctx
        .computeStore_all()
        .iter()
        .map(|cs| {
            let field = cs
                .identifier().map_or_else(|| make_data_ref(&cs.get_text()), |id| extract_data_ref_from_identifier(&id));
            ArithTarget {
                field,
                rounded: cs.ROUNDED().is_some(),
            }
        })
        .collect();

    let expression = ctx
        .arithmeticExpression()
        .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))), |expr| extract_arith_expr(&expr));

    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    Statement::Compute(ComputeStatement {
        targets,
        expression,
        on_size_error,
        not_on_size_error,
    })
}

fn extract_if(ctx: &IfStatementContext<'_>) -> Statement {
    let condition = ctx
        .condition()
        .map_or(Condition::ConditionName(make_data_ref("TRUE")), |c| extract_condition(&c));

    let then_body: Vec<Statement> = ctx
        .ifThen()
        .map(|then_ctx| {
            // Check for NEXT SENTENCE
            if then_ctx.NEXT().is_some() && then_ctx.SENTENCE().is_some() {
                return vec![Statement::NextSentence];
            }
            then_ctx
                .statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    let else_body: Vec<Statement> = ctx
        .ifElse()
        .map(|else_ctx| {
            if else_ctx.NEXT().is_some() && else_ctx.SENTENCE().is_some() {
                return vec![Statement::NextSentence];
            }
            else_ctx
                .statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    Statement::If(IfStatement {
        condition,
        then_body,
        else_body,
    })
}

fn extract_evaluate(ctx: &EvaluateStatementContext<'_>) -> Statement {
    // Extract subjects
    let mut subjects = Vec::new();
    if let Some(sel) = ctx.evaluateSelect() {
        subjects.push(extract_evaluate_subject(&sel));
    }
    for also in ctx.evaluateAlsoSelect_all() {
        if let Some(sel) = also.evaluateSelect() {
            subjects.push(extract_evaluate_subject(&sel));
        }
    }

    // Extract WHEN branches
    let when_branches: Vec<WhenBranch> = ctx
        .evaluateWhenPhrase_all()
        .iter()
        .map(|wp| {
            let values: Vec<WhenValue> = wp
                .evaluateWhen_all()
                .iter()
                .map(|w| {
                    // Use structured parse tree: evaluateWhen -> evaluateCondition
                    if let Some(cond_ctx) = w.evaluateCondition() {
                        extract_when_value_from_condition(&cond_ctx)
                    } else {
                        // Fallback to text-based
                        let text = w.get_text().to_uppercase();
                        let text = text.strip_prefix("WHEN").unwrap_or(&text).trim();
                        if text == "ANY" || text == "OTHER" {
                            WhenValue::Any
                        } else {
                            WhenValue::Value(extract_identifier_or_literal_from_text(text))
                        }
                    }
                })
                .collect();
            let body: Vec<Statement> = wp
                .statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect();
            WhenBranch { values, body }
        })
        .collect();

    // Extract WHEN OTHER
    let when_other: Vec<Statement> = ctx
        .evaluateWhenOther()
        .map(|wo| {
            wo.statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    Statement::Evaluate(EvaluateStatement {
        subjects,
        when_branches,
        when_other,
    })
}

fn extract_perform(ctx: &PerformStatementContext<'_>) -> Statement {
    if let Some(inline) = ctx.performInlineStatement() {
        let loop_type = inline
            .performType()
            .map_or(PerformLoopType::Once, |pt| extract_perform_type(&pt));
        let body: Vec<Statement> = inline
            .statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect();
        Statement::Perform(PerformStatement {
            target: None,
            thru: None,
            loop_type,
            body,
        })
    } else if let Some(proc) = ctx.performProcedureStatement() {
        let names = proc.procedureName_all();
        let target = names
            .first()
            .map(|n| PerformTarget {
                name: n.get_text().trim().to_uppercase(),
            });
        let thru = if proc.THROUGH().is_some() || proc.THRU().is_some() {
            names.get(1).map(|n| n.get_text().trim().to_uppercase())
        } else {
            None
        };
        let loop_type = proc
            .performType()
            .map_or(PerformLoopType::Once, |pt| extract_perform_type(&pt));
        Statement::Perform(PerformStatement {
            target,
            thru,
            loop_type,
            body: Vec::new(),
        })
    } else {
        Statement::Perform(PerformStatement {
            target: None,
            thru: None,
            loop_type: PerformLoopType::Once,
            body: Vec::new(),
        })
    }
}

fn extract_goto(ctx: &GoToStatementContext<'_>) -> Statement {
    if let Some(simple) = ctx.goToStatementSimple() {
        let target = simple
            .procedureName()
            .map(|pn| pn.get_text().trim().to_uppercase())
            .unwrap_or_default();
        Statement::GoTo(GoToStatement {
            targets: vec![target],
            depending: None,
        })
    } else if let Some(dep) = ctx.goToDependingOnStatement() {
        // Extract targets from procedureName() children (not fragile text split)
        let targets: Vec<String> = dep
            .procedureName_all()
            .iter()
            .map(|pn| pn.get_text().trim().to_uppercase())
            .collect();

        // Extract the DEPENDING ON identifier
        let depending = dep
            .identifier()
            .map(|id| extract_data_ref_from_identifier(&id));

        Statement::GoTo(GoToStatement { targets, depending })
    } else {
        Statement::GoTo(GoToStatement {
            targets: Vec::new(),
            depending: None,
        })
    }
}

fn extract_stop(_ctx: &StopStatementContext<'_>) -> Statement {
    Statement::StopRun
}

fn extract_initialize(ctx: &InitializeStatementContext<'_>) -> Statement {
    let targets: Vec<DataReference> = ctx
        .identifier_all()
        .iter()
        .map(|id| extract_data_ref_from_identifier(id))
        .collect();

    let mut replacing = Vec::new();
    if let Some(repl_phrase) = ctx.initializeReplacingPhrase() {
        for repl_by in repl_phrase.initializeReplacingBy_all() {
            let category = if repl_by.NUMERIC().is_some() {
                InitializeCategory::Numeric
            } else if repl_by.ALPHANUMERIC().is_some() {
                InitializeCategory::Alphanumeric
            } else if repl_by.ALPHABETIC().is_some() {
                InitializeCategory::Alphabetic
            } else if repl_by.ALPHANUMERIC_EDITED().is_some() {
                InitializeCategory::AlphanumericEdited
            } else if repl_by.NUMERIC_EDITED().is_some() {
                InitializeCategory::NumericEdited
            } else if repl_by.NATIONAL().is_some() {
                InitializeCategory::National
            } else {
                continue;
            };
            let value = if let Some(id) = repl_by.identifier() {
                Operand::DataRef(extract_data_ref_from_identifier(&id))
            } else if let Some(lit) = repl_by.literal() {
                extract_literal_operand(&lit)
            } else {
                continue;
            };
            replacing.push(InitializeReplacing { category, value });
        }
    }

    Statement::Initialize(InitializeStatement {
        targets,
        replacing,
    })
}

fn extract_call(ctx: &CallStatementContext<'_>) -> Statement {
    let program = if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else {
        Operand::Literal(Literal::Alphanumeric(String::new()))
    };

    // Extract USING parameters
    let using = if let Some(using_phrase) = ctx.callUsingPhrase() {
        extract_call_using_params(&using_phrase)
    } else {
        Vec::new()
    };

    // Extract RETURNING / GIVING field
    let returning = ctx.callGivingPhrase().and_then(|gp| {
        gp.identifier()
            .map(|id| extract_data_ref_from_identifier(&id))
    });

    // ON EXCEPTION / ON OVERFLOW (synonym in CALL context)
    let mut on_exception = extract_on_exception_stmts(ctx.onExceptionClause().as_deref());
    if on_exception.is_empty() {
        on_exception = extract_on_overflow_stmts(ctx.onOverflowPhrase().as_deref());
    }

    // NOT ON EXCEPTION
    let not_on_exception =
        extract_not_on_exception_stmts(ctx.notOnExceptionClause().as_deref());

    Statement::Call(CallStatement {
        program,
        using,
        returning,
        on_exception,
        not_on_exception,
    })
}

fn extract_call_using_params(
    ctx: &CallUsingPhraseContext<'_>,
) -> Vec<CallParam> {
    let mut params = Vec::new();

    for param_ctx in ctx.callUsingParameter_all() {
        // BY REFERENCE (default)
        if let Some(ref_phrase) = param_ctx.callByReferencePhrase() {
            for ref_item in ref_phrase.callByReference_all() {
                if ref_item.OMITTED().is_some() {
                    params.push(CallParam {
                        mode: PassingMode::Omitted,
                        operand: None,
                    });
                } else if let Some(id) = ref_item.identifier() {
                    params.push(CallParam {
                        mode: PassingMode::ByReference,
                        operand: Some(Operand::DataRef(
                            extract_data_ref_from_identifier(&id),
                        )),
                    });
                } else if let Some(lit) = ref_item.literal() {
                    params.push(CallParam {
                        mode: PassingMode::ByReference,
                        operand: Some(extract_literal_operand(&lit)),
                    });
                }
            }
        }

        // BY CONTENT
        if let Some(content_phrase) = param_ctx.callByContentPhrase() {
            for content_item in content_phrase.callByContent_all() {
                if content_item.OMITTED().is_some() {
                    params.push(CallParam {
                        mode: PassingMode::Omitted,
                        operand: None,
                    });
                } else if let Some(id) = content_item.identifier() {
                    params.push(CallParam {
                        mode: PassingMode::ByContent,
                        operand: Some(Operand::DataRef(
                            extract_data_ref_from_identifier(&id),
                        )),
                    });
                } else if let Some(lit) = content_item.literal() {
                    params.push(CallParam {
                        mode: PassingMode::ByContent,
                        operand: Some(extract_literal_operand(&lit)),
                    });
                }
            }
        }

        // BY VALUE
        if let Some(value_phrase) = param_ctx.callByValuePhrase() {
            for value_item in value_phrase.callByValue_all() {
                if let Some(id) = value_item.identifier() {
                    params.push(CallParam {
                        mode: PassingMode::ByValue,
                        operand: Some(Operand::DataRef(
                            extract_data_ref_from_identifier(&id),
                        )),
                    });
                } else if let Some(lit) = value_item.literal() {
                    params.push(CallParam {
                        mode: PassingMode::ByValue,
                        operand: Some(extract_literal_operand(&lit)),
                    });
                }
            }
        }
    }

    params
}

fn extract_on_exception_stmts(
    ctx: Option<&OnExceptionClauseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_on_exception_stmts(
    ctx: Option<&NotOnExceptionClauseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_cancel(ctx: &CancelStatementContext<'_>) -> Statement {
    let programs: Vec<Operand> = ctx
        .cancelCall_all()
        .iter()
        .filter_map(|cc| {
            if let Some(id) = cc.identifier() {
                Some(Operand::DataRef(extract_data_ref_from_identifier(&id)))
            } else if let Some(lit) = cc.literal() {
                Some(extract_literal_operand(&lit))
            } else {
                // libraryName fallback -- use text
                cc.libraryName().map(|ln| {
                    Operand::Literal(Literal::Alphanumeric(
                        ln.get_text().trim().to_uppercase(),
                    ))
                })
            }
        })
        .collect();

    Statement::Cancel(CancelStatement { programs })
}

/// Extract a SET statement from the parse tree.
///
/// Handles:
/// - SET target TO value/identifier (`SetAction::To`)
/// - SET condition TO TRUE/FALSE (`SetAction::ToBool`)
/// - SET target UP BY value (`SetAction::UpBy`)
/// - SET target DOWN BY value (`SetAction::DownBy`)
fn extract_set(ctx: &SetStatementContext<'_>) -> Statement {
    // SET ... TO ... (one or more setToStatement children)
    let to_stmts = ctx.setToStatement_all();
    if !to_stmts.is_empty() {
        // Use the first setToStatement (most common case: single SET ... TO ...)
        let to_stmt = &to_stmts[0];

        // Extract targets from setTo children
        let targets: Vec<DataReference> = to_stmt
            .setTo_all()
            .iter()
            .filter_map(|st| {
                st.identifier()
                    .map(|id| extract_data_ref_from_identifier(&id))
            })
            .collect();

        // Extract value from setToValue children
        let values = to_stmt.setToValue_all();
        let action = if let Some(val) = values.first() {
            let text = val.get_text().to_uppercase();
            if text == "TRUE" || val.ON().is_some() {
                SetAction::ToBool(true)
            } else if text == "FALSE" || val.OFF().is_some() {
                SetAction::ToBool(false)
            } else if let Some(id) = val.identifier() {
                SetAction::To(Operand::DataRef(extract_data_ref_from_identifier(&id)))
            } else if let Some(lit) = val.literal() {
                SetAction::To(extract_literal_operand(&lit))
            } else {
                // Fallback: parse text as identifier or literal
                SetAction::To(extract_identifier_or_literal_from_text(&text))
            }
        } else {
            // No value -- shouldn't happen but handle gracefully
            SetAction::To(Operand::Literal(Literal::Numeric("0".to_string())))
        };

        return Statement::Set(SetStatement { targets, action });
    }

    // SET ... UP BY / DOWN BY
    if let Some(updown) = ctx.setUpDownByStatement() {
        let targets: Vec<DataReference> = updown
            .setTo_all()
            .iter()
            .filter_map(|st| {
                st.identifier()
                    .map(|id| extract_data_ref_from_identifier(&id))
            })
            .collect();

        let value = if let Some(by_val) = updown.setByValue() {
            if let Some(id) = by_val.identifier() {
                Operand::DataRef(extract_data_ref_from_identifier(&id))
            } else if let Some(lit) = by_val.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Numeric(by_val.get_text().trim().to_string()))
            }
        } else {
            Operand::Literal(Literal::Numeric("1".to_string()))
        };

        let action = if updown.UP().is_some() {
            SetAction::UpBy(value)
        } else {
            SetAction::DownBy(value)
        };

        return Statement::Set(SetStatement { targets, action });
    }

    // Fallback
    Statement::Set(SetStatement {
        targets: Vec::new(),
        action: SetAction::To(Operand::Literal(Literal::Numeric("0".to_string()))),
    })
}

/// Extract a START statement from the parse tree.
fn extract_start(ctx: &StartStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let key_condition = ctx.startKey().map(|sk| {
        let key = sk
            .qualifiedDataName().map_or_else(|| make_data_ref("UNKNOWN-KEY"), |qdn| extract_data_ref_from_qualified(&qdn));

        // Determine comparison operator from grammar tokens
        let op = if sk.EQUAL().is_some() || sk.EQUALCHAR().is_some() {
            ComparisonOp::Equal
        } else if sk.NOT().is_some() {
            // NOT LESS THAN or NOT LESSTHANCHAR -> GreaterOrEqual
            ComparisonOp::GreaterOrEqual
        } else if sk.OR().is_some() || sk.MORETHANOREQUAL().is_some() {
            // GREATER THAN OR EQUAL TO or >= -> GreaterOrEqual
            ComparisonOp::GreaterOrEqual
        } else if sk.GREATER().is_some() || sk.MORETHANCHAR().is_some() {
            ComparisonOp::GreaterThan
        } else {
            // Default to EQUAL if we can't determine
            ComparisonOp::Equal
        };

        StartKeyCondition { key, op }
    });

    let invalid_key = extract_invalid_key_stmts(
        ctx.invalidKeyPhrase().as_deref(),
    );

    let not_invalid_key = extract_not_invalid_key_stmts(
        ctx.notInvalidKeyPhrase().as_deref(),
    );

    Statement::Start(StartStatement {
        file_name,
        key_condition,
        invalid_key,
        not_invalid_key,
    })
}

fn extract_accept(ctx: &AcceptStatementContext<'_>) -> Statement {
    // Extract target identifier
    let target = if let Some(id) = ctx.identifier() {
        extract_data_ref_from_identifier(&id)
    } else {
        let text = ctx.get_text().to_uppercase();
        let name = text
            .strip_prefix("ACCEPT")
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        make_data_ref(&name)
    };

    // Determine source (FROM DATE/TIME/DAY/DAY-OF-WEEK or SYSIN)
    let from = if let Some(date_ctx) = ctx.acceptFromDateStatement() {
        if date_ctx.DATE().is_some() {
            if date_ctx.YYYYMMDD().is_some() {
                AcceptSource::DateYyyyMmDd
            } else {
                AcceptSource::Date
            }
        } else if date_ctx.DAY().is_some() {
            if date_ctx.YYYYDDD().is_some() {
                AcceptSource::DayYyyyDdd
            } else {
                AcceptSource::Day
            }
        } else if date_ctx.DAY_OF_WEEK().is_some() {
            AcceptSource::DayOfWeek
        } else if date_ctx.TIME().is_some() || date_ctx.TIMER().is_some() {
            AcceptSource::Time
        } else {
            AcceptSource::Date
        }
    } else {
        AcceptSource::Sysin
    };

    Statement::Accept(AcceptStatement { target, from })
}

// ---------------------------------------------------------------------------
// File I/O statement extractors
// ---------------------------------------------------------------------------

fn extract_open(ctx: &OpenStatementContext<'_>) -> Statement {
    let mut files = Vec::new();

    // OPEN INPUT file-name ...
    for input_stmt in ctx.openInputStatement_all() {
        for open_input in input_stmt.openInput_all() {
            if let Some(fname_ctx) = open_input.fileName() {
                files.push(OpenFile {
                    mode: OpenMode::Input,
                    file_name: fname_ctx.get_text().trim().to_uppercase(),
                });
            }
        }
    }

    // OPEN OUTPUT file-name ...
    for output_stmt in ctx.openOutputStatement_all() {
        for open_output in output_stmt.openOutput_all() {
            if let Some(fname_ctx) = open_output.fileName() {
                files.push(OpenFile {
                    mode: OpenMode::Output,
                    file_name: fname_ctx.get_text().trim().to_uppercase(),
                });
            }
        }
    }

    // OPEN I-O file-name ...
    for io_stmt in ctx.openIOStatement_all() {
        for fname_ctx in io_stmt.fileName_all() {
            files.push(OpenFile {
                mode: OpenMode::IoMode,
                file_name: fname_ctx.get_text().trim().to_uppercase(),
            });
        }
    }

    // OPEN EXTEND file-name ...
    for extend_stmt in ctx.openExtendStatement_all() {
        for fname_ctx in extend_stmt.fileName_all() {
            files.push(OpenFile {
                mode: OpenMode::Extend,
                file_name: fname_ctx.get_text().trim().to_uppercase(),
            });
        }
    }

    Statement::Open(OpenStatement { files })
}

fn extract_close(ctx: &CloseStatementContext<'_>) -> Statement {
    let files: Vec<String> = ctx
        .closeFile_all()
        .iter()
        .filter_map(|cf| cf.fileName().map(|f| f.get_text().trim().to_uppercase()))
        .collect();
    Statement::Close(CloseStatement { files })
}

fn extract_read(ctx: &ReadStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let into = ctx.readInto().and_then(|ri| {
        ri.identifier()
            .map(|id| extract_data_ref_from_identifier(&id))
    });

    let key = ctx.readKey().and_then(|rk| {
        rk.qualifiedDataName()
            .map(|qdn| extract_data_ref_from_qualified(&qdn))
    });

    let at_end = ctx
        .atEndPhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    let not_at_end = ctx
        .notAtEndPhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    let invalid_key = extract_invalid_key_stmts(ctx.invalidKeyPhrase().as_deref());
    let not_invalid_key = extract_not_invalid_key_stmts(ctx.notInvalidKeyPhrase().as_deref());

    Statement::Read(ReadStatement {
        file_name,
        into,
        key,
        at_end,
        not_at_end,
        invalid_key,
        not_invalid_key,
    })
}

fn extract_write(ctx: &WriteStatementContext<'_>) -> Statement {
    let record_name = ctx
        .recordName()
        .map(|rn| rn.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let from = ctx.writeFromPhrase().and_then(|wfp| {
        wfp.identifier().map(|id| extract_data_ref_from_identifier(&id))
    });

    let advancing = ctx.writeAdvancingPhrase().and_then(|wap| {
        if wap.writeAdvancingPage().is_some() {
            Some(Advancing::Page)
        } else if let Some(lines_ctx) = wap.writeAdvancingLines() {
            let text = lines_ctx.get_text();
            // Extract the line count: strip "LINE" / "LINES" keywords
            let clean = text
                .to_uppercase()
                .replace("LINE", "")
                .replace('S', "")
                .trim()
                .to_string();
            let op = extract_identifier_or_literal_from_text(&clean);
            Some(Advancing::Lines(op))
        } else {
            None
        }
    });

    let invalid_key = extract_invalid_key_stmts(ctx.invalidKeyPhrase().as_deref());
    let not_invalid_key = extract_not_invalid_key_stmts(ctx.notInvalidKeyPhrase().as_deref());

    let at_eop = ctx
        .writeAtEndOfPagePhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    let not_at_eop = ctx
        .writeNotAtEndOfPagePhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(s))
                .collect()
        })
        .unwrap_or_default();

    Statement::Write(WriteStatement {
        record_name,
        from,
        advancing,
        invalid_key,
        not_invalid_key,
        at_eop,
        not_at_eop,
    })
}

fn extract_rewrite(ctx: &RewriteStatementContext<'_>) -> Statement {
    let record_name = ctx
        .recordName()
        .map(|rn| rn.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let from = ctx.rewriteFrom().and_then(|rf| {
        rf.identifier()
            .map(|id| extract_data_ref_from_identifier(&id))
    });

    let invalid_key = extract_invalid_key_stmts(ctx.invalidKeyPhrase().as_deref());
    let not_invalid_key = extract_not_invalid_key_stmts(ctx.notInvalidKeyPhrase().as_deref());

    Statement::Rewrite(RewriteStatement {
        record_name,
        from,
        invalid_key,
        not_invalid_key,
    })
}

fn extract_delete(ctx: &DeleteStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let invalid_key = extract_invalid_key_stmts(ctx.invalidKeyPhrase().as_deref());
    let not_invalid_key = extract_not_invalid_key_stmts(ctx.notInvalidKeyPhrase().as_deref());

    Statement::Delete(DeleteStatement {
        file_name,
        invalid_key,
        not_invalid_key,
    })
}

fn extract_invalid_key_stmts(
    ctx: Option<&InvalidKeyPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_invalid_key_stmts(
    ctx: Option<&NotInvalidKeyPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Perform type extraction
// ---------------------------------------------------------------------------

fn extract_perform_type(ctx: &PerformTypeContext<'_>) -> PerformLoopType {
    if let Some(times_ctx) = ctx.performTimes() {
        let count = if let Some(id) = times_ctx.identifier() {
            Operand::DataRef(extract_data_ref_from_identifier(&id))
        } else if let Some(int_lit) = times_ctx.integerLiteral() {
            Operand::Literal(Literal::Numeric(int_lit.get_text().trim().to_string()))
        } else {
            Operand::Literal(Literal::Numeric("1".to_string()))
        };
        PerformLoopType::Times(count)
    } else if let Some(until_ctx) = ctx.performUntil() {
        let test_before = until_ctx
            .performTestClause()
            .is_none_or(|tc| !tc.get_text().to_uppercase().contains("AFTER"));
        let condition = until_ctx
            .condition()
            .map_or(Condition::ConditionName(make_data_ref("TRUE")), |c| extract_condition(&c));
        PerformLoopType::Until {
            test_before,
            condition,
        }
    } else if let Some(varying_ctx) = ctx.performVarying() {
        extract_perform_varying(&varying_ctx)
    } else {
        PerformLoopType::Once
    }
}

fn extract_perform_varying(
    ctx: &PerformVaryingContext<'_>,
) -> PerformLoopType {
    let test_before = ctx
        .performTestClause()
        .is_none_or(|tc| !tc.get_text().to_uppercase().contains("AFTER"));

    // Navigate the ANTLR parse tree properly:
    // performVarying -> performVaryingClause -> performVaryingPhrase
    //   -> identifier (counter), performFrom, performBy, performUntil -> condition
    let vc = ctx.performVaryingClause();
    let phrase = vc.as_ref().and_then(|vc| vc.performVaryingPhrase());

    // Extract counter name from the identifier child of performVaryingPhrase
    let counter_name = phrase
        .as_ref()
        .and_then(|p| p.identifier()).map_or_else(|| "I".to_string(), |id| id.get_text().to_uppercase());

    // Extract FROM value from performFrom child
    let from_text = phrase
        .as_ref()
        .and_then(|p| p.performFrom())
        .map_or_else(|| "1".to_string(), |pf| {
            // performFrom = FROM (identifier | literal | arithmeticExpression)
            if let Some(id) = pf.identifier() {
                id.get_text().to_uppercase()
            } else if let Some(lit) = pf.literal() {
                lit.get_text()
            } else if let Some(arith) = pf.arithmeticExpression() {
                arith.get_text()
            } else {
                "1".to_string()
            }
        });

    // Extract BY value from performBy child
    let by_text = phrase
        .as_ref()
        .and_then(|p| p.performBy())
        .map_or_else(|| "1".to_string(), |pb| {
            if let Some(id) = pb.identifier() {
                id.get_text().to_uppercase()
            } else if let Some(lit) = pb.literal() {
                lit.get_text()
            } else if let Some(arith) = pb.arithmeticExpression() {
                arith.get_text()
            } else {
                "1".to_string()
            }
        });

    // Extract UNTIL condition from performUntil -> condition child
    let condition = phrase
        .as_ref()
        .and_then(|p| p.performUntil())
        .and_then(|pu| {
            if let Some(cond_ctx) = pu.condition() {
                Some(extract_condition(&cond_ctx))
            } else {
                // Fallback: parse from text
                let pu_text = pu.get_text().to_uppercase();
                if let Some(until_pos) = pu_text.find("UNTIL") {
                    let cond_text = pu_text[until_pos + 5..].trim();
                    Some(parse_simple_condition(cond_text))
                } else {
                    None
                }
            }
        })
        .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));

    PerformLoopType::Varying {
        test_before,
        counter: make_data_ref(&counter_name),
        from: extract_operand_from_identifier_or_literal_ctx(&from_text),
        by: extract_operand_from_identifier_or_literal_ctx(&by_text),
        until: condition,
        after: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Condition extraction
// ---------------------------------------------------------------------------

fn extract_condition(ctx: &ConditionContext<'_>) -> Condition {
    let base = ctx
        .combinableCondition()
        .map_or(Condition::ConditionName(make_data_ref("TRUE")), |cc| extract_combinable_condition(&cc));

    // Track implied subject/operator for abbreviated conditions (e.g., A = 50 OR 75)
    let (mut implied_subject, mut implied_op) = extract_implied_subject_op(&base);

    // Process AND/OR chains
    let and_or_list = ctx.andOrCondition_all();
    if and_or_list.is_empty() {
        return base;
    }

    let mut result = base;
    for ao in &and_or_list {
        let right = if let Some(cc) = ao.combinableCondition() {
            let cond = extract_combinable_condition(&cc);
            // Update implied subject/op from this condition
            let (subj, op) = extract_implied_subject_op(&cond);
            if subj.is_some() {
                implied_subject = subj;
                implied_op = op;
            }
            cond
        } else {
            // Abbreviated condition: expand abbreviations using implied subject
            let abbrevs = ao.abbreviation_all();
            if !abbrevs.is_empty() {
                expand_abbreviation(&abbrevs[0], &implied_subject, implied_op)
            } else {
                Condition::ConditionName(make_data_ref("TRUE"))
            }
        };
        if ao.AND().is_some() {
            result = Condition::And(Box::new(result), Box::new(right));
        } else {
            result = Condition::Or(Box::new(result), Box::new(right));
        }
    }
    result
}

/// Extract implied subject and operator from a comparison condition.
/// Returns (Some(subject_operand), operator) if the condition is a Comparison.
fn extract_implied_subject_op(cond: &Condition) -> (Option<Operand>, ComparisonOp) {
    match cond {
        Condition::Comparison { left, op, .. } => (Some(left.clone()), *op),
        _ => (None, ComparisonOp::Equal),
    }
}

/// Expand an abbreviated condition using the implied subject and operator.
/// For example: `75` in `A = 50 OR 75` expands to `A = 75`.
fn expand_abbreviation(
    abbrev: &AbbreviationContext<'_>,
    implied_subject: &Option<Operand>,
    implied_op: ComparisonOp,
) -> Condition {
    // Determine operator: use abbreviation's operator if present, else implied
    let op = abbrev
        .relationalOperator()
        .map_or(implied_op, |ro| extract_relational_op(&ro));

    let negated = abbrev.NOT().is_some();

    // Get the value from the abbreviation
    let right = abbrev
        .arithmeticExpression()
        .map(|ae| arith_expr_to_operand(&extract_arith_expr(&ae)))
        .unwrap_or(Operand::Literal(Literal::Numeric("0".to_string())));

    let left = implied_subject
        .clone()
        .unwrap_or(Operand::Literal(Literal::Numeric("0".to_string())));

    let cond = Condition::Comparison { left, op, right };
    if negated {
        Condition::Not(Box::new(cond))
    } else {
        cond
    }
}

fn extract_combinable_condition(
    ctx: &CombinableConditionContext<'_>,
) -> Condition {
    let negated = ctx.NOT().is_some();
    let cond = ctx
        .simpleCondition()
        .map_or(Condition::ConditionName(make_data_ref("TRUE")), |sc| extract_simple_condition(&sc));

    if negated {
        Condition::Not(Box::new(cond))
    } else {
        cond
    }
}

fn extract_simple_condition(
    ctx: &SimpleConditionContext<'_>,
) -> Condition {
    // Parenthesized condition
    if ctx.LPARENCHAR().is_some() {
        if let Some(inner) = ctx.condition() {
            return extract_condition(&inner);
        }
    }

    // Relation condition
    if let Some(rel) = ctx.relationCondition() {
        return extract_relation_condition(&rel);
    }

    // Class condition
    if let Some(cls) = ctx.classCondition() {
        return extract_class_condition(&cls);
    }

    // Condition name (88-level)
    if let Some(cnr) = ctx.conditionNameReference() {
        let name = cnr.get_text().trim().to_uppercase();
        return Condition::ConditionName(make_data_ref(&name));
    }

    // Fallback
    Condition::ConditionName(make_data_ref("TRUE"))
}

fn extract_relation_condition(
    ctx: &RelationConditionContext<'_>,
) -> Condition {
    if let Some(arith_cmp) = ctx.relationArithmeticComparison() {
        let exprs = arith_cmp.arithmeticExpression_all();
        let left = exprs
            .first()
            .map_or(Operand::Literal(Literal::Numeric("0".to_string())), |e| arith_expr_to_operand(&extract_arith_expr(e)));
        let right = exprs
            .get(1)
            .map_or(Operand::Literal(Literal::Numeric("0".to_string())), |e| arith_expr_to_operand(&extract_arith_expr(e)));
        let op = arith_cmp
            .relationalOperator()
            .map_or(ComparisonOp::Equal, |ro| extract_relational_op(&ro));
        Condition::Comparison { left, op, right }
    } else if let Some(sign_cond) = ctx.relationSignCondition() {
        let text = sign_cond.get_text().to_uppercase();
        let sign = if text.contains("POSITIVE") {
            SignCondition::Positive
        } else if text.contains("NEGATIVE") {
            SignCondition::Negative
        } else {
            SignCondition::Zero
        };
        // Extract field from the text before IS
        let field_text = text
            .split("IS")
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        Condition::SignTest {
            field: make_data_ref(&field_text),
            sign,
        }
    } else {
        Condition::ConditionName(make_data_ref("TRUE"))
    }
}

fn extract_relational_op(ctx: &RelationalOperatorContext<'_>) -> ComparisonOp {
    // Check for combined operators first
    if ctx.NOTEQUALCHAR().is_some() {
        return ComparisonOp::NotEqual;
    }
    if ctx.MORETHANOREQUAL().is_some() {
        return ComparisonOp::GreaterOrEqual;
    }
    if ctx.LESSTHANOREQUAL().is_some() {
        return ComparisonOp::LessOrEqual;
    }

    let has_not = ctx.NOT().is_some();
    let has_greater = ctx.GREATER().is_some() || ctx.MORETHANCHAR().is_some();
    let has_less = ctx.LESS().is_some() || ctx.LESSTHANCHAR().is_some();
    let has_equal = ctx.EQUAL().is_some() || ctx.EQUALCHAR().is_some();

    let has_or_equal = ctx.OR().is_some() && has_equal;
    if has_not && has_equal {
        ComparisonOp::NotEqual
    } else if has_not && has_greater || (has_less && has_or_equal) {
        ComparisonOp::LessOrEqual
    } else if has_not && has_less || (has_greater && has_or_equal) {
        ComparisonOp::GreaterOrEqual
    } else if has_greater {
        ComparisonOp::GreaterThan
    } else if has_less {
        ComparisonOp::LessThan
    } else {
        ComparisonOp::Equal
    }
}

fn extract_class_condition(ctx: &ClassConditionContext<'_>) -> Condition {
    let field = ctx
        .identifier().map_or_else(|| make_data_ref("UNKNOWN"), |id| extract_data_ref_from_identifier(&id));

    let class = if ctx.NUMERIC().is_some() {
        ClassCondition::Numeric
    } else if ctx.ALPHABETIC_LOWER().is_some() {
        ClassCondition::AlphabeticLower
    } else if ctx.ALPHABETIC_UPPER().is_some() {
        ClassCondition::AlphabeticUpper
    } else {
        ClassCondition::Alphabetic
    };

    let cond = Condition::ClassTest { field, class };

    if ctx.NOT().is_some() {
        Condition::Not(Box::new(cond))
    } else {
        cond
    }
}

// ---------------------------------------------------------------------------
// Arithmetic expression extraction
// ---------------------------------------------------------------------------

fn extract_arith_expr(ctx: &ArithmeticExpressionContext<'_>) -> ArithExpr {
    let base = ctx
        .multDivs()
        .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))), |md| extract_mult_divs(&md));

    let plus_minus_list = ctx.plusMinus_all();
    if plus_minus_list.is_empty() {
        return base;
    }

    let mut result = base;
    for pm in &plus_minus_list {
        let right = pm
            .multDivs()
            .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))), |md| extract_mult_divs(&md));
        let op = if pm.PLUSCHAR().is_some() {
            ArithOp::Add
        } else {
            ArithOp::Subtract
        };
        result = ArithExpr::BinaryOp {
            left: Box::new(result),
            op,
            right: Box::new(right),
        };
    }
    result
}

fn extract_mult_divs(ctx: &MultDivsContext<'_>) -> ArithExpr {
    let base = ctx
        .powers()
        .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))), |p| extract_powers(&p));

    let md_list = ctx.multDiv_all();
    if md_list.is_empty() {
        return base;
    }

    let mut result = base;
    for md in &md_list {
        let right = md
            .powers()
            .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))), |p| extract_powers(&p));
        let text = md.get_text().to_uppercase();
        let op = if text.starts_with('*') || text.starts_with("MULT") {
            ArithOp::Multiply
        } else {
            ArithOp::Divide
        };
        result = ArithExpr::BinaryOp {
            left: Box::new(result),
            op,
            right: Box::new(right),
        };
    }
    result
}

fn extract_powers(ctx: &PowersContext<'_>) -> ArithExpr {
    let has_negate = ctx.MINUSCHAR().is_some();

    let base = ctx
        .basis()
        .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))), |b| extract_basis(&b));

    let power_list = ctx.power_all();
    let mut result = base;
    for pw in &power_list {
        let right = pw
            .basis()
            .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))), |b| extract_basis(&b));
        result = ArithExpr::BinaryOp {
            left: Box::new(result),
            op: ArithOp::Power,
            right: Box::new(right),
        };
    }

    if has_negate {
        ArithExpr::Negate(Box::new(result))
    } else {
        result
    }
}

fn extract_basis(ctx: &BasisContext<'_>) -> ArithExpr {
    if let Some(expr) = ctx.arithmeticExpression() {
        ArithExpr::Paren(Box::new(extract_arith_expr(&expr)))
    } else if let Some(id) = ctx.identifier() {
        ArithExpr::Operand(extract_operand_from_identifier(&id))
    } else if let Some(lit) = ctx.literal() {
        ArithExpr::Operand(extract_literal_operand(&lit))
    } else {
        ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            ctx.get_text().trim().to_string(),
        )))
    }
}

// ---------------------------------------------------------------------------
// Helper functions: identifier/literal extraction
// ---------------------------------------------------------------------------

/// Extract a `RefMod` from a referenceModifier context.
fn extract_ref_mod(ctx: &ReferenceModifierContext<'_>) -> RefMod {
    let offset = ctx
        .characterPosition()
        .and_then(|cp| cp.arithmeticExpression())
        .map_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "1".to_string(),
        ))), |ae| extract_arith_expr(&ae));
    let length = ctx
        .length()
        .and_then(|l| l.arithmeticExpression())
        .map(|ae| Box::new(extract_arith_expr(&ae)));
    RefMod {
        offset: Box::new(offset),
        length,
    }
}

/// Extract an `Operand` from an `IdentifierContext`, handling function calls.
fn extract_operand_from_identifier(ctx: &IdentifierContext<'_>) -> Operand {
    // Check for FUNCTION call first
    if let Some(fc) = ctx.functionCall() {
        let func_name = fc.functionName()
            .map_or_else(|| fc.get_text().to_uppercase(), |fn_ctx| fn_ctx.get_text().to_uppercase());
        // Strip FUNCTION prefix if present
        let func_name = func_name.strip_prefix("FUNCTION").unwrap_or(&func_name).trim().to_string();
        let arguments: Vec<Operand> = fc.argument_all()
            .iter()
            .map(|arg| {
                if let Some(lit) = arg.literal() {
                    extract_literal_operand(&lit)
                } else if let Some(id) = arg.identifier() {
                    extract_operand_from_identifier(&id)
                } else if let Some(qdn) = arg.qualifiedDataName() {
                    Operand::DataRef(extract_data_ref_from_qualified(&qdn))
                } else {
                    extract_identifier_or_literal_from_text(&arg.get_text())
                }
            })
            .collect();
        return Operand::Function(FunctionCall {
            name: func_name,
            arguments,
        });
    }
    Operand::DataRef(extract_data_ref_from_identifier(ctx))
}

/// Extract a `DataReference` from an `IdentifierContext`.
fn extract_data_ref_from_identifier(
    ctx: &IdentifierContext<'_>,
) -> DataReference {
    if let Some(qdn) = ctx.qualifiedDataName() {
        extract_data_ref_from_qualified(&qdn)
    } else if let Some(tc) = ctx.tableCall() {
        let base = tc
            .qualifiedDataName().map_or_else(|| make_data_ref(&tc.get_text()), |qdn| extract_data_ref_from_qualified(&qdn));
        // Add subscripts from table call
        let subscripts: Vec<Subscript> = tc
            .subscript__all()
            .iter()
            .map(|s| {
                let text = s.get_text().trim().to_string();
                if let Ok(n) = text.parse::<i32>() {
                    Subscript::IntLiteral(n)
                } else {
                    Subscript::DataRef(make_data_ref(&text))
                }
            })
            .collect();
        // Extract reference modification if present
        let ref_mod = tc.referenceModifier().map(|rm| extract_ref_mod(&rm));
        DataReference {
            subscripts,
            ref_mod,
            ..base
        }
    } else {
        make_data_ref(&ctx.get_text())
    }
}

/// Extract a `DataReference` from a `QualifiedDataNameContext`.
fn extract_data_ref_from_qualified(
    ctx: &QualifiedDataNameContext<'_>,
) -> DataReference {
    if let Some(fmt1) = ctx.qualifiedDataNameFormat1() {
        let name = fmt1
            .dataName()
            .map(|dn| dn.get_text().trim().to_uppercase())
            .or_else(|| {
                fmt1.conditionName()
                    .map(|cn| cn.get_text().trim().to_uppercase())
            })
            .unwrap_or_default();
        let qualifiers: Vec<String> = fmt1
            .qualifiedInData_all()
            .iter()
            .map(|qid| {
                qid.get_text()
                    .trim()
                    .to_uppercase()
                    .replace("IN", "")
                    .replace("OF", "")
                    .trim()
                    .to_string()
            })
            .collect();
        DataReference {
            name,
            qualifiers,
            subscripts: Vec::new(),
            ref_mod: None,
        }
    } else {
        make_data_ref(&ctx.get_text())
    }
}

/// Extract an Operand from a `LiteralContext`.
fn extract_literal_operand(ctx: &LiteralContext<'_>) -> Operand {
    if let Some(num) = ctx.numericLiteral() {
        Operand::Literal(Literal::Numeric(num.get_text().trim().to_string()))
    } else if let Some(fig) = ctx.figurativeConstant() {
        let text = fig.get_text().to_uppercase();
        let fc = if text.starts_with("ALL") {
            // ALL "x" -- extract the literal after ALL
            let rest = &fig.get_text()[3..];
            let inner = strip_cobol_quotes(rest);
            if inner.is_empty() {
                FigurativeConstant::Spaces
            } else {
                FigurativeConstant::All(inner)
            }
        } else if text.contains("SPACE") {
            FigurativeConstant::Spaces
        } else if text.contains("ZERO") {
            FigurativeConstant::Zeros
        } else if text.contains("HIGH") {
            FigurativeConstant::HighValues
        } else if text.contains("LOW") {
            FigurativeConstant::LowValues
        } else if text.contains("QUOTE") {
            FigurativeConstant::Quotes
        } else if text.contains("NULL") {
            FigurativeConstant::Nulls
        } else {
            FigurativeConstant::Spaces
        };
        Operand::Literal(Literal::Figurative(fc))
    } else if let Some(nn) = ctx.NONNUMERICLITERAL() {
        let raw = nn.get_text();
        let stripped = strip_cobol_quotes(&raw);
        Operand::Literal(Literal::Alphanumeric(stripped))
    } else {
        Operand::Literal(Literal::Alphanumeric(ctx.get_text()))
    }
}

/// Extract operand from `MoveToSendingArea`.
fn extract_operand_from_sending_area(
    ctx: &MoveToSendingAreaContext<'_>,
) -> Operand {
    if let Some(id) = ctx.identifier() {
        extract_operand_from_identifier(&id)
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else {
        extract_identifier_or_literal_from_text(&ctx.get_text())
    }
}

/// Extract operand from `AddFrom` context.
fn extract_operand_from_add_from(ctx: &AddFromContext<'_>) -> Operand {
    if let Some(id) = ctx.identifier() {
        extract_operand_from_identifier(&id)
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else {
        extract_identifier_or_literal_from_text(&ctx.get_text())
    }
}

/// Extract evaluate subject from `EvaluateSelectContext`.
fn extract_evaluate_subject(
    ctx: &EvaluateSelectContext<'_>,
) -> EvaluateSubject {
    let text = ctx.get_text().to_uppercase();
    if text.trim() == "TRUE" {
        return EvaluateSubject::Bool(true);
    }
    if text.trim() == "FALSE" {
        return EvaluateSubject::Bool(false);
    }

    if let Some(id) = ctx.identifier() {
        EvaluateSubject::Expr(extract_operand_from_identifier(&id))
    } else if let Some(lit) = ctx.literal() {
        EvaluateSubject::Expr(extract_literal_operand(&lit))
    } else {
        EvaluateSubject::Expr(extract_identifier_or_literal_from_text(&text))
    }
}

/// Extract a `WhenValue` from a structured `EvaluateConditionContext`.
fn extract_when_value_from_condition(ctx: &EvaluateConditionContext<'_>) -> WhenValue {
    // Alt 1: ANY
    if ctx.ANY().is_some() {
        return WhenValue::Any;
    }

    // Alt 3: condition (boolean expression like WS-A > 75)
    if let Some(cond) = ctx.condition() {
        return WhenValue::Condition(extract_condition(&cond));
    }

    // Alt 4: booleanLiteral (TRUE/FALSE)
    if let Some(bl) = ctx.booleanLiteral() {
        let text = bl.get_text().to_uppercase();
        if text == "TRUE" {
            return WhenValue::Value(Operand::Literal(Literal::Alphanumeric("TRUE".to_string())));
        }
        return WhenValue::Value(Operand::Literal(Literal::Alphanumeric("FALSE".to_string())));
    }

    // Alt 2: [NOT] evaluateValue [THRU evaluateThrough]
    if let Some(val_ctx) = ctx.evaluateValue() {
        let operand = extract_evaluate_value_operand(&val_ctx);

        // Check for THRU range
        if let Some(through_ctx) = ctx.evaluateThrough() {
            if let Some(high_val) = through_ctx.evaluateValue() {
                let high_operand = extract_evaluate_value_operand(&high_val);
                return WhenValue::Range {
                    low: operand,
                    high: high_operand,
                };
            }
        }

        return WhenValue::Value(operand);
    }

    // Fallback: use raw text
    let text = ctx.get_text().to_uppercase();
    WhenValue::Value(extract_identifier_or_literal_from_text(&text))
}

/// Extract an `Operand` from an `EvaluateValueContext`.
fn extract_evaluate_value_operand(ctx: &EvaluateValueContext<'_>) -> Operand {
    if let Some(id) = ctx.identifier() {
        extract_operand_from_identifier(&id)
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&lit)
    } else if let Some(arith) = ctx.arithmeticExpression() {
        let expr = extract_arith_expr(&arith);
        arith_expr_to_operand(&expr)
    } else {
        let text = ctx.get_text();
        extract_identifier_or_literal_from_text(&text)
    }
}

// ---------------------------------------------------------------------------
// Size error phrase extraction
// ---------------------------------------------------------------------------

fn extract_size_error_stmts(
    ctx: Option<&OnSizeErrorPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_size_error_stmts(
    ctx: Option<&NotOnSizeErrorPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Text-based helper functions
// ---------------------------------------------------------------------------

/// Create a simple `DataReference` from a name string.
/// Handles qualified names (X IN Y) and subscripts (X(SUB)).
fn make_data_ref(name: &str) -> DataReference {
    let clean = name.trim().to_uppercase();

    // Extract subscripts from parenthesized notation: FIELD(SUB1, SUB2)
    let (base_text, subscripts) = if let Some(paren_pos) = clean.find('(') {
        let base = clean[..paren_pos].trim().to_string();
        let subs_text = clean[paren_pos + 1..].trim_end_matches(')').trim();
        let subs: Vec<Subscript> = subs_text
            .split(',')
            .map(|s| {
                let s = s.trim();
                if let Ok(n) = s.parse::<i32>() {
                    Subscript::IntLiteral(n)
                } else {
                    Subscript::DataRef(DataReference {
                        name: s.to_string(),
                        qualifiers: Vec::new(),
                        subscripts: Vec::new(),
                        ref_mod: None,
                    })
                }
            })
            .collect();
        (base, subs)
    } else {
        (clean.clone(), Vec::new())
    };

    // Handle qualified names (X IN Y)
    let parts: Vec<&str> = base_text.split(' ').collect();
    let (data_name, qualifiers) = if parts.len() >= 3 {
        let mut quals = Vec::new();
        let mut i = 2;
        while i < parts.len() {
            if parts.get(i.wrapping_sub(1)).is_some_and(|p| *p == "IN" || *p == "OF") {
                quals.push(parts[i].to_string());
            }
            i += 1;
        }
        (parts[0].to_string(), quals)
    } else {
        (base_text.clone(), Vec::new())
    };

    DataReference {
        name: data_name,
        qualifiers,
        subscripts,
        ref_mod: None,
    }
}

/// Convert an Operand to a DataReference. For DataRef operands, unwraps the
/// inner DataReference. For literals, creates a DataReference from the literal text.
fn operand_to_data_ref(op: &Operand) -> DataReference {
    match op {
        Operand::DataRef(dr) => dr.clone(),
        Operand::Literal(Literal::Numeric(n)) => make_data_ref(n),
        Operand::Literal(Literal::Alphanumeric(s)) => make_data_ref(s),
        _ => make_data_ref("0"),
    }
}

/// Extract data ref from identifier text (stripping ROUNDED if present).
fn extract_data_ref_from_identifier_text(text: &str) -> DataReference {
    let clean = text
        .trim()
        .to_uppercase()
        .replace("ROUNDED", "")
        .trim()
        .to_string();
    make_data_ref(&clean)
}

/// Extract giving phrase targets from text (fallback for non-ANTLR contexts).
#[cfg(test)]
fn extract_giving_phrase_targets(text: &str) -> Vec<ArithTarget> {
    let upper = text.trim().to_uppercase();
    let clean = upper
        .strip_prefix("GIVING")
        .unwrap_or(&upper)
        .trim();
    // Split on whitespace, treating ROUNDED as a modifier
    let mut targets = Vec::new();
    let tokens: Vec<&str> = clean.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        let name = tokens[i];
        let rounded = tokens.get(i + 1).is_some_and(|t| *t == "ROUNDED");
        targets.push(ArithTarget {
            field: make_data_ref(name),
            rounded,
        });
        i += if rounded { 2 } else { 1 };
    }
    targets
}

/// Extract DIVIDE GIVING targets using ANTLR context (proper identifier/ROUNDED).
fn extract_divide_giving_targets(ctx: &DivideGivingPhraseContext<'_>) -> Vec<ArithTarget> {
    ctx.divideGiving_all()
        .iter()
        .map(|g| ArithTarget {
            field: g.identifier().map_or_else(
                || extract_data_ref_from_identifier_text(&g.get_text()),
                |id| extract_data_ref_from_identifier_text(&id.get_text()),
            ),
            rounded: g.ROUNDED().is_some(),
        })
        .collect()
}

/// Parse an operand from raw text (identifier or literal).
fn extract_identifier_or_literal_from_text(text: &str) -> Operand {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Operand::Literal(Literal::Alphanumeric(String::new()));
    }

    let upper = trimmed.to_uppercase();

    // Check figurative constants
    match upper.as_str() {
        "SPACES" | "SPACE" => {
            return Operand::Literal(Literal::Figurative(FigurativeConstant::Spaces));
        }
        "ZEROS" | "ZEROES" | "ZERO" => {
            return Operand::Literal(Literal::Figurative(FigurativeConstant::Zeros));
        }
        "HIGH-VALUES" | "HIGH-VALUE" => {
            return Operand::Literal(Literal::Figurative(FigurativeConstant::HighValues));
        }
        "LOW-VALUES" | "LOW-VALUE" => {
            return Operand::Literal(Literal::Figurative(FigurativeConstant::LowValues));
        }
        _ => {}
    }

    // Check if quoted string
    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        return Operand::Literal(Literal::Alphanumeric(strip_cobol_quotes(trimmed)));
    }

    // Check if numeric
    if trimmed
        .bytes()
        .all(|b| b.is_ascii_digit() || b == b'+' || b == b'-' || b == b'.')
    {
        return Operand::Literal(Literal::Numeric(trimmed.to_string()));
    }

    // Treat as data reference
    Operand::DataRef(make_data_ref(trimmed))
}

/// Alias for text-based operand extraction.
fn extract_operand_from_identifier_or_literal_ctx(text: &str) -> Operand {
    extract_identifier_or_literal_from_text(text)
}

/// Convert an `ArithExpr` to an Operand (for simple expressions).
fn arith_expr_to_operand(expr: &ArithExpr) -> Operand {
    match expr {
        ArithExpr::Operand(op) => op.clone(),
        _ => {
            // For complex expressions, use the text representation
            Operand::Literal(Literal::Numeric("0".to_string()))
        }
    }
}

/// Parse a simple condition from text (fallback for PERFORM VARYING).
fn parse_simple_condition(text: &str) -> Condition {
    let trimmed = text.trim();
    let upper = trimmed.to_uppercase();

    // Try to detect comparison operators
    for (op_str, op) in &[
        (">=", ComparisonOp::GreaterOrEqual),
        ("<=", ComparisonOp::LessOrEqual),
        ("NOT=", ComparisonOp::NotEqual),
        (">", ComparisonOp::GreaterThan),
        ("<", ComparisonOp::LessThan),
        ("=", ComparisonOp::Equal),
    ] {
        if let Some(pos) = upper.find(op_str) {
            let left = trimmed[..pos].trim();
            let right = trimmed[pos + op_str.len()..].trim();
            return Condition::Comparison {
                left: extract_identifier_or_literal_from_text(left),
                op: *op,
                right: extract_identifier_or_literal_from_text(right),
            };
        }
    }

    // Check for GREATER THAN, LESS THAN, EQUAL TO patterns
    if upper.contains("GREATER") || upper.contains("LESS") || upper.contains("EQUAL") {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 {
            let left = parts[0];
            let right = parts.last().unwrap_or(&"0");
            let op = if upper.contains("NOT") && upper.contains("EQUAL") {
                ComparisonOp::NotEqual
            } else if upper.contains("GREATER") && upper.contains("EQUAL") {
                ComparisonOp::GreaterOrEqual
            } else if upper.contains("LESS") && upper.contains("EQUAL") {
                ComparisonOp::LessOrEqual
            } else if upper.contains("GREATER") {
                ComparisonOp::GreaterThan
            } else if upper.contains("LESS") {
                ComparisonOp::LessThan
            } else {
                ComparisonOp::Equal
            };
            return Condition::Comparison {
                left: extract_identifier_or_literal_from_text(left),
                op,
                right: extract_identifier_or_literal_from_text(right),
            };
        }
    }

    // Fallback: treat as condition name
    Condition::ConditionName(make_data_ref(trimmed))
}

/// Strip COBOL quotes from a string literal.
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

// ---------------------------------------------------------------------------
// Phase 3 statement extractors: SORT, MERGE, RELEASE, RETURN, INSPECT, STRING, UNSTRING
// ---------------------------------------------------------------------------

fn extract_sort(ctx: &SortStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    // Extract sort keys
    let mut keys = Vec::new();
    for key_clause in ctx.sortOnKeyClause_all() {
        let ascending = key_clause.ASCENDING().is_some();
        for qdn in key_clause.qualifiedDataName_all() {
            keys.push(SortKey {
                ascending,
                field: make_data_ref(&qdn.get_text()),
            });
        }
    }

    let duplicates = ctx.sortDuplicatesPhrase().is_some();

    let collating = ctx
        .sortCollatingSequencePhrase()
        .map(|c| c.get_text().trim().to_uppercase());

    // Input: USING files or INPUT PROCEDURE
    let input = if let Some(input_proc) = ctx.sortInputProcedurePhrase() {
        let name = input_proc
            .procedureName()
            .map(|p| p.get_text().trim().to_uppercase())
            .unwrap_or_default();
        let thru = input_proc
            .sortInputThrough()
            .and_then(|t| t.procedureName().map(|p| p.get_text().trim().to_uppercase()));
        SortInput::InputProcedure { name, thru }
    } else {
        let files: Vec<String> = ctx
            .sortUsing_all()
            .iter()
            .flat_map(|su| su.fileName_all())
            .map(|f| f.get_text().trim().to_uppercase())
            .collect();
        SortInput::Using(files)
    };

    // Output: GIVING files or OUTPUT PROCEDURE
    let output = if let Some(output_proc) = ctx.sortOutputProcedurePhrase() {
        let name = output_proc
            .procedureName()
            .map(|p| p.get_text().trim().to_uppercase())
            .unwrap_or_default();
        let thru = output_proc
            .sortOutputThrough()
            .and_then(|t| t.procedureName().map(|p| p.get_text().trim().to_uppercase()));
        SortOutput::OutputProcedure { name, thru }
    } else {
        let files: Vec<String> = ctx
            .sortGivingPhrase_all()
            .iter()
            .flat_map(|sg| sg.sortGiving_all())
            .filter_map(|g| g.fileName())
            .map(|f| f.get_text().trim().to_uppercase())
            .collect();
        SortOutput::Giving(files)
    };

    Statement::Sort(SortStatement {
        file_name,
        keys,
        duplicates,
        collating,
        input,
        output,
    })
}

fn extract_merge(ctx: &MergeStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let mut keys = Vec::new();
    for key_clause in ctx.mergeOnKeyClause_all() {
        let ascending = key_clause.ASCENDING().is_some();
        for qdn in key_clause.qualifiedDataName_all() {
            keys.push(SortKey {
                ascending,
                field: make_data_ref(&qdn.get_text()),
            });
        }
    }

    let collating = ctx
        .mergeCollatingSequencePhrase()
        .map(|c| c.get_text().trim().to_uppercase());

    let using: Vec<String> = ctx
        .mergeUsing_all()
        .iter()
        .flat_map(|mu| mu.fileName_all())
        .map(|f| f.get_text().trim().to_uppercase())
        .collect();

    let output = if let Some(output_proc) = ctx.mergeOutputProcedurePhrase() {
        let name = output_proc
            .procedureName()
            .map(|p| p.get_text().trim().to_uppercase())
            .unwrap_or_default();
        let thru = output_proc
            .mergeOutputThrough()
            .and_then(|t| t.procedureName().map(|p| p.get_text().trim().to_uppercase()));
        SortOutput::OutputProcedure { name, thru }
    } else {
        let files: Vec<String> = ctx
            .mergeGivingPhrase_all()
            .iter()
            .flat_map(|mg| mg.mergeGiving_all())
            .filter_map(|g| g.fileName())
            .map(|f| f.get_text().trim().to_uppercase())
            .collect();
        SortOutput::Giving(files)
    };

    Statement::Merge(MergeStatement {
        file_name,
        keys,
        collating,
        using,
        output,
    })
}

fn extract_release(ctx: &ReleaseStatementContext<'_>) -> Statement {
    let record_name = ctx
        .recordName()
        .map(|r| r.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let from = ctx
        .qualifiedDataName()
        .map(|q| make_data_ref(&q.get_text()));

    Statement::Release(ReleaseStatement { record_name, from })
}

fn extract_return(ctx: &ReturnStatementContext<'_>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let into = ctx
        .returnInto()
        .and_then(|ri| ri.qualifiedDataName().map(|q| make_data_ref(&q.get_text())));

    let at_end = extract_at_end_stmts(ctx.atEndPhrase().as_deref());
    let not_at_end = extract_not_at_end_stmts(ctx.notAtEndPhrase().as_deref());

    Statement::Return(ReturnStatement {
        file_name,
        into,
        at_end,
        not_at_end,
    })
}

fn extract_at_end_stmts(ctx: Option<&AtEndPhraseContext<'_>>) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_at_end_stmts(
    ctx: Option<&NotAtEndPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_on_overflow_stmts(
    ctx: Option<&OnOverflowPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_on_overflow_stmts(
    ctx: Option<&NotOnOverflowPhraseContext<'_>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_inspect(ctx: &InspectStatementContext<'_>) -> Statement {
    let target = ctx
        .identifier().map_or_else(|| make_data_ref(""), |id| extract_data_ref_from_identifier(&id));

    let mut tallying = Vec::new();
    let mut replacing = Vec::new();
    let mut converting = None;

    // INSPECT ... TALLYING
    if let Some(tp) = ctx.inspectTallyingPhrase() {
        tallying = extract_inspect_tallying_items(&tp);
    }

    // INSPECT ... REPLACING
    if let Some(rp) = ctx.inspectReplacingPhrase() {
        replacing = extract_inspect_replacing_items(&rp);
    }

    // INSPECT ... TALLYING ... REPLACING (combined)
    if let Some(trp) = ctx.inspectTallyingReplacingPhrase() {
        for inspect_for in trp.inspectFor_all() {
            tallying.extend(extract_inspect_for_items(&inspect_for));
        }
        for rep_phrase in trp.inspectReplacingPhrase_all() {
            replacing.extend(extract_inspect_replacing_items(&rep_phrase));
        }
    }

    // INSPECT ... CONVERTING
    if let Some(cp) = ctx.inspectConvertingPhrase() {
        let from_text = if let Some(id) = cp.identifier() {
            extract_operand_from_identifier_or_literal_ctx(&id.get_text())
        } else if let Some(lit) = cp.literal() {
            extract_literal_operand(&lit)
        } else {
            Operand::Literal(Literal::Alphanumeric(String::new()))
        };

        let to_text = cp
            .inspectTo()
            .map_or(Operand::Literal(Literal::Alphanumeric(String::new())), |to| {
                if let Some(id) = to.identifier() {
                    extract_operand_from_identifier_or_literal_ctx(&id.get_text())
                } else if let Some(lit) = to.literal() {
                    extract_literal_operand(&lit)
                } else {
                    Operand::Literal(Literal::Alphanumeric(String::new()))
                }
            });

        converting = Some(InspectConverting {
            from: from_text,
            to: to_text,
        });
    }

    Statement::Inspect(InspectStatement {
        target,
        tallying,
        replacing,
        converting,
    })
}

fn extract_inspect_tallying_items(
    tp: &InspectTallyingPhraseContext<'_>,
) -> Vec<InspectTallying> {
    let mut items = Vec::new();
    for inspect_for in tp.inspectFor_all() {
        items.extend(extract_inspect_for_items(&inspect_for));
    }
    items
}

fn extract_inspect_for_items(
    inspect_for: &InspectForContext<'_>,
) -> Vec<InspectTallying> {
    let counter = inspect_for
        .identifier().map_or_else(|| make_data_ref(""), |id| extract_data_ref_from_identifier(&id));

    let mut items = Vec::new();

    // CHARACTERS
    for chars_ctx in inspect_for.inspectCharacters_all() {
        items.push(InspectTallying {
            counter: counter.clone(),
            what: InspectWhat::Characters,
        });
        let _ = chars_ctx; // before/after deferred to codegen
    }

    // ALL / LEADING
    for all_leading in inspect_for.inspectAllLeadings_all() {
        let is_all = all_leading.ALL().is_some();
        for al in all_leading.inspectAllLeading_all() {
            let pattern = if let Some(id) = al.identifier() {
                extract_operand_from_identifier_or_literal_ctx(&id.get_text())
            } else if let Some(lit) = al.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Alphanumeric(String::new()))
            };

            let what = if is_all {
                InspectWhat::All(pattern)
            } else {
                InspectWhat::Leading(pattern)
            };

            items.push(InspectTallying {
                counter: counter.clone(),
                what,
            });
        }
    }

    items
}

fn extract_inspect_replacing_items(
    rp: &InspectReplacingPhraseContext<'_>,
) -> Vec<InspectReplacing> {
    let mut items = Vec::new();

    // CHARACTERS BY ...
    for chars_ctx in rp.inspectReplacingCharacters_all() {
        let by = chars_ctx
            .inspectBy()
            .map_or(Operand::Literal(Literal::Alphanumeric(String::new())), |by| {
                if let Some(id) = by.identifier() {
                    extract_operand_from_identifier_or_literal_ctx(&id.get_text())
                } else if let Some(lit) = by.literal() {
                    extract_literal_operand(&lit)
                } else {
                    Operand::Literal(Literal::Alphanumeric(String::new()))
                }
            });

        items.push(InspectReplacing {
            what: InspectWhat::Characters,
            by,
        });
    }

    // ALL / LEADING / FIRST ... BY ...
    for all_leadings in rp.inspectReplacingAllLeadings_all() {
        let is_all = all_leadings.ALL().is_some();
        let is_first = all_leadings.FIRST().is_some();

        for ral in all_leadings.inspectReplacingAllLeading_all() {
            let pattern = if let Some(id) = ral.identifier() {
                extract_operand_from_identifier_or_literal_ctx(&id.get_text())
            } else if let Some(lit) = ral.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Alphanumeric(String::new()))
            };

            let by = ral
                .inspectBy()
                .map_or(Operand::Literal(Literal::Alphanumeric(String::new())), |by| {
                    if let Some(id) = by.identifier() {
                        extract_operand_from_identifier_or_literal_ctx(&id.get_text())
                    } else if let Some(lit) = by.literal() {
                        extract_literal_operand(&lit)
                    } else {
                        Operand::Literal(Literal::Alphanumeric(String::new()))
                    }
                });

            let what = if is_first {
                InspectWhat::First(pattern)
            } else if is_all {
                InspectWhat::All(pattern)
            } else {
                InspectWhat::Leading(pattern)
            };

            items.push(InspectReplacing { what, by });
        }
    }

    items
}

fn extract_string(ctx: &StringStatementContext<'_>) -> Statement {
    let into = ctx
        .stringIntoPhrase()
        .and_then(|ip| ip.identifier().map(|id| extract_data_ref_from_identifier(&id)))
        .unwrap_or_else(|| make_data_ref(""));

    let mut sources = Vec::new();
    for sp in ctx.stringSendingPhrase_all() {
        // Each sending phrase has sending items + a delimiter phrase
        let delimiter = sp
            .stringDelimitedByPhrase()
            .map_or(StringDelimiter::Size, |dp| {
                if dp.SIZE().is_some() {
                    StringDelimiter::Size
                } else if let Some(id) = dp.identifier() {
                    StringDelimiter::Literal(extract_operand_from_identifier_or_literal_ctx(
                        &id.get_text(),
                    ))
                } else if let Some(lit) = dp.literal() {
                    StringDelimiter::Literal(extract_literal_operand(&lit))
                } else {
                    StringDelimiter::Size
                }
            });

        for sending in sp.stringSending_all() {
            let operand = if let Some(id) = sending.identifier() {
                extract_operand_from_identifier_or_literal_ctx(&id.get_text())
            } else if let Some(lit) = sending.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Alphanumeric(String::new()))
            };

            sources.push(StringSource {
                operand,
                delimited_by: delimiter.clone(),
            });
        }
    }

    let pointer = ctx
        .stringWithPointerPhrase()
        .and_then(|wp| wp.qualifiedDataName().map(|q| make_data_ref(&q.get_text())));

    let on_overflow = extract_on_overflow_stmts(ctx.onOverflowPhrase().as_deref());
    let not_on_overflow = extract_not_on_overflow_stmts(ctx.notOnOverflowPhrase().as_deref());

    Statement::String(StringStatement {
        sources,
        into,
        pointer,
        on_overflow,
        not_on_overflow,
    })
}

fn extract_unstring(ctx: &UnstringStatementContext<'_>) -> Statement {
    let source = ctx
        .unstringSendingPhrase()
        .and_then(|sp| sp.identifier().map(|id| extract_data_ref_from_identifier(&id)))
        .unwrap_or_else(|| make_data_ref(""));

    // Extract delimiters
    let mut delimiters = Vec::new();
    if let Some(sp) = ctx.unstringSendingPhrase() {
        if let Some(dbp) = sp.unstringDelimitedByPhrase() {
            let all = dbp.ALL().is_some();
            let value = if let Some(id) = dbp.identifier() {
                extract_operand_from_identifier_or_literal_ctx(&id.get_text())
            } else if let Some(lit) = dbp.literal() {
                extract_literal_operand(&lit)
            } else {
                Operand::Literal(Literal::Alphanumeric(String::new()))
            };
            delimiters.push(UnstringDelimiter { value, all });

            // OR delimiters
            for or_phrase in sp.unstringOrAllPhrase_all() {
                let or_all = or_phrase.ALL().is_some();
                let or_value = if let Some(id) = or_phrase.identifier() {
                    extract_operand_from_identifier_or_literal_ctx(&id.get_text())
                } else if let Some(lit) = or_phrase.literal() {
                    extract_literal_operand(&lit)
                } else {
                    Operand::Literal(Literal::Alphanumeric(String::new()))
                };
                delimiters.push(UnstringDelimiter {
                    value: or_value,
                    all: or_all,
                });
            }
        }
    }

    // Extract INTO targets
    let mut into = Vec::new();
    if let Some(ip) = ctx.unstringIntoPhrase() {
        for ui in ip.unstringInto_all() {
            let target = ui
                .identifier().map_or_else(|| make_data_ref(""), |id| extract_data_ref_from_identifier(&id));

            let delimiter_in = ui
                .unstringDelimiterIn()
                .and_then(|di| di.identifier().map(|id| extract_data_ref_from_identifier(&id)));

            let count_in = ui
                .unstringCountIn()
                .and_then(|ci| ci.identifier().map(|id| extract_data_ref_from_identifier(&id)));

            into.push(UnstringInto {
                target,
                delimiter_in,
                count_in,
            });
        }
    }

    let pointer = ctx
        .unstringWithPointerPhrase()
        .and_then(|wp| wp.qualifiedDataName().map(|q| make_data_ref(&q.get_text())));

    let tallying = ctx
        .unstringTallyingPhrase()
        .and_then(|tp| tp.qualifiedDataName().map(|q| make_data_ref(&q.get_text())));

    let on_overflow = extract_on_overflow_stmts(ctx.onOverflowPhrase().as_deref());
    let not_on_overflow = extract_not_on_overflow_stmts(ctx.notOnOverflowPhrase().as_deref());

    Statement::Unstring(UnstringStatement {
        source,
        delimiters,
        into,
        pointer,
        tallying,
        on_overflow,
        not_on_overflow,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_data_ref_simple() {
        let dr = make_data_ref("WS-FIELD");
        assert_eq!(dr.name, "WS-FIELD");
        assert!(dr.qualifiers.is_empty());
    }

    #[test]
    fn extract_literal_from_text_numeric() {
        match extract_identifier_or_literal_from_text("42") {
            Operand::Literal(Literal::Numeric(s)) => assert_eq!(s, "42"),
            other => panic!("expected Numeric, got {other:?}"),
        }
    }

    #[test]
    fn extract_literal_from_text_spaces() {
        match extract_identifier_or_literal_from_text("SPACES") {
            Operand::Literal(Literal::Figurative(FigurativeConstant::Spaces)) => {}
            other => panic!("expected Spaces, got {other:?}"),
        }
    }

    #[test]
    fn extract_literal_from_text_identifier() {
        match extract_identifier_or_literal_from_text("WS-COUNTER") {
            Operand::DataRef(dr) => assert_eq!(dr.name, "WS-COUNTER"),
            other => panic!("expected DataRef, got {other:?}"),
        }
    }

    #[test]
    fn parse_simple_condition_comparison() {
        match parse_simple_condition("WS-X>10") {
            Condition::Comparison { left, op, right } => {
                assert_eq!(op, ComparisonOp::GreaterThan);
                match left {
                    Operand::DataRef(dr) => assert_eq!(dr.name, "WS-X"),
                    other => panic!("expected DataRef for left, got {other:?}"),
                }
                match right {
                    Operand::Literal(Literal::Numeric(n)) => assert_eq!(n, "10"),
                    other => panic!("expected Numeric for right, got {other:?}"),
                }
            }
            other => panic!("expected Comparison, got {other:?}"),
        }
    }

    #[test]
    fn giving_phrase_targets() {
        let targets = extract_giving_phrase_targets("GIVING WS-RESULT ROUNDED WS-OTHER");
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].field.name, "WS-RESULT");
        assert!(targets[0].rounded);
        assert_eq!(targets[1].field.name, "WS-OTHER");
        assert!(!targets[1].rounded);
    }

    #[test]
    fn strip_quotes_single() {
        assert_eq!(strip_cobol_quotes("'HELLO'"), "HELLO");
    }
}
