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
            .map(|h| h.get_text().trim().to_uppercase())
            .map(|s| {
                // Extract just the section name (before "SECTION")
                s.split_whitespace().next().unwrap_or("").to_string()
            });
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
                if let Some(stmt) = extract_statement(&*stmt_ctx) {
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

/// Extract a Statement from a StatementContext by checking which child rule matched.
fn extract_statement<'input>(ctx: &StatementContext<'input>) -> Option<Statement> {
    if let Some(c) = ctx.moveStatement() {
        return Some(extract_move(&*c));
    }
    if let Some(c) = ctx.displayStatement() {
        return Some(extract_display(&*c));
    }
    if let Some(c) = ctx.addStatement() {
        return Some(extract_add(&*c));
    }
    if let Some(c) = ctx.subtractStatement() {
        return Some(extract_subtract(&*c));
    }
    if let Some(c) = ctx.multiplyStatement() {
        return Some(extract_multiply(&*c));
    }
    if let Some(c) = ctx.divideStatement() {
        return Some(extract_divide(&*c));
    }
    if let Some(c) = ctx.computeStatement() {
        return Some(extract_compute(&*c));
    }
    if let Some(c) = ctx.ifStatement() {
        return Some(extract_if(&*c));
    }
    if let Some(c) = ctx.evaluateStatement() {
        return Some(extract_evaluate(&*c));
    }
    if let Some(c) = ctx.performStatement() {
        return Some(extract_perform(&*c));
    }
    if let Some(c) = ctx.goToStatement() {
        return Some(extract_goto(&*c));
    }
    if let Some(c) = ctx.stopStatement() {
        return Some(extract_stop(&*c));
    }
    if ctx.gobackStatement().is_some() {
        return Some(Statement::GoBack);
    }
    if let Some(c) = ctx.initializeStatement() {
        return Some(extract_initialize(&*c));
    }
    if ctx.continueStatement().is_some() {
        return Some(Statement::Continue);
    }
    if let Some(c) = ctx.callStatement() {
        return Some(extract_call(&*c));
    }
    if let Some(c) = ctx.acceptStatement() {
        return Some(extract_accept(&*c));
    }
    if ctx.exitStatement().is_some() {
        return Some(Statement::ExitProgram);
    }
    if let Some(c) = ctx.openStatement() {
        return Some(extract_open(&*c));
    }
    if let Some(c) = ctx.closeStatement() {
        return Some(extract_close(&*c));
    }
    if let Some(c) = ctx.readStatement() {
        return Some(extract_read(&*c));
    }
    if let Some(c) = ctx.writeStatement() {
        return Some(extract_write(&*c));
    }
    if let Some(c) = ctx.rewriteStatement() {
        return Some(extract_rewrite(&*c));
    }
    if let Some(c) = ctx.deleteStatement() {
        return Some(extract_delete(&*c));
    }
    // Unsupported statement -- skip
    None
}

// ---------------------------------------------------------------------------
// Individual statement extractors
// ---------------------------------------------------------------------------

fn extract_move<'input>(ctx: &MoveStatementContext<'input>) -> Statement {
    if let Some(corr_ctx) = ctx.moveCorrespondingToStatement() {
        let source = corr_ctx
            .moveCorrespondingToSendingArea()
            .map(|s| extract_identifier_or_literal_from_text(&s.get_text()))
            .unwrap_or_else(|| Operand::Literal(Literal::Alphanumeric(String::new())));
        let destinations: Vec<DataReference> = corr_ctx
            .identifier_all()
            .iter()
            .map(|id| extract_data_ref_from_identifier(&**id))
            .collect();
        Statement::Move(MoveStatement {
            corresponding: true,
            source,
            destinations,
        })
    } else if let Some(move_to) = ctx.moveToStatement() {
        let source = move_to
            .moveToSendingArea()
            .map(|s| extract_operand_from_sending_area(&*s))
            .unwrap_or_else(|| Operand::Literal(Literal::Alphanumeric(String::new())));
        let destinations: Vec<DataReference> = move_to
            .identifier_all()
            .iter()
            .map(|id| extract_data_ref_from_identifier(&**id))
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

fn extract_display<'input>(ctx: &DisplayStatementContext<'input>) -> Statement {
    let items: Vec<Operand> = ctx
        .displayOperand_all()
        .iter()
        .map(|op| {
            if let Some(id) = op.identifier() {
                Operand::DataRef(extract_data_ref_from_identifier(&*id))
            } else if let Some(lit) = op.literal() {
                extract_literal_operand(&*lit)
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

fn extract_add<'input>(ctx: &AddStatementContext<'input>) -> Statement {
    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    if let Some(giving_ctx) = ctx.addToGivingStatement() {
        let operands: Vec<Operand> = giving_ctx
            .addFrom_all()
            .iter()
            .map(|f| extract_operand_from_add_from(&**f))
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
            .map(|f| extract_operand_from_add_from(&**f))
            .collect();
        let to: Vec<ArithTarget> = to_ctx
            .addTo_all()
            .iter()
            .map(|t| ArithTarget {
                field: extract_data_ref_from_identifier_text(&t.get_text()),
                rounded: t.get_text().to_uppercase().contains("ROUNDED"),
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

fn extract_subtract<'input>(ctx: &SubtractStatementContext<'input>) -> Statement {
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
                field: extract_data_ref_from_identifier_text(&m.get_text()),
                rounded: m.get_text().to_uppercase().contains("ROUNDED"),
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

fn extract_multiply<'input>(ctx: &MultiplyStatementContext<'input>) -> Statement {
    let multiplicand = if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&*id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&*lit)
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
                field: extract_data_ref_from_identifier_text(&r.get_text()),
                rounded: r.get_text().to_uppercase().contains("ROUNDED"),
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
                field: extract_data_ref_from_identifier_text(&o.get_text()),
                rounded: o.get_text().to_uppercase().contains("ROUNDED"),
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

fn extract_divide<'input>(ctx: &DivideStatementContext<'input>) -> Statement {
    let operand = if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&*id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&*lit)
    } else {
        Operand::Literal(Literal::Numeric("0".to_string()))
    };

    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());
    let remainder = ctx.divideRemainder().map(|r| ArithTarget {
        field: extract_data_ref_from_identifier_text(&r.get_text()),
        rounded: false,
    });

    if let Some(into_giving) = ctx.divideIntoGivingStatement() {
        let into_text = into_giving
            .identifier()
            .map(|id| id.get_text())
            .or_else(|| into_giving.literal().map(|l| l.get_text()))
            .unwrap_or_default();
        let giving = into_giving
            .divideGivingPhrase()
            .map(|gp| extract_giving_phrase_targets(&gp.get_text()))
            .unwrap_or_default();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::Into,
            into: vec![ArithTarget {
                field: make_data_ref(&into_text),
                rounded: false,
            }],
            giving,
            remainder,
            on_size_error,
            not_on_size_error,
        })
    } else if let Some(by_giving) = ctx.divideByGivingStatement() {
        let by_text = by_giving
            .identifier()
            .map(|id| id.get_text())
            .or_else(|| by_giving.literal().map(|l| l.get_text()))
            .unwrap_or_default();
        let giving = by_giving
            .divideGivingPhrase()
            .map(|gp| extract_giving_phrase_targets(&gp.get_text()))
            .unwrap_or_default();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::By,
            into: vec![ArithTarget {
                field: make_data_ref(&by_text),
                rounded: false,
            }],
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
                field: extract_data_ref_from_identifier_text(&d.get_text()),
                rounded: d.get_text().to_uppercase().contains("ROUNDED"),
            })
            .collect();
        Statement::Divide(DivideStatement {
            operand,
            direction: DivideDirection::Into,
            into,
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
            giving: Vec::new(),
            remainder: None,
            on_size_error: Vec::new(),
            not_on_size_error: Vec::new(),
        })
    }
}

fn extract_compute<'input>(ctx: &ComputeStatementContext<'input>) -> Statement {
    let targets: Vec<ArithTarget> = ctx
        .computeStore_all()
        .iter()
        .map(|cs| {
            let field = cs
                .identifier()
                .map(|id| extract_data_ref_from_identifier(&*id))
                .unwrap_or_else(|| make_data_ref(&cs.get_text()));
            ArithTarget {
                field,
                rounded: cs.ROUNDED().is_some(),
            }
        })
        .collect();

    let expression = ctx
        .arithmeticExpression()
        .map(|expr| extract_arith_expr(&*expr))
        .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))));

    let on_size_error = extract_size_error_stmts(ctx.onSizeErrorPhrase().as_deref());
    let not_on_size_error = extract_not_size_error_stmts(ctx.notOnSizeErrorPhrase().as_deref());

    Statement::Compute(ComputeStatement {
        targets,
        expression,
        on_size_error,
        not_on_size_error,
    })
}

fn extract_if<'input>(ctx: &IfStatementContext<'input>) -> Statement {
    let condition = ctx
        .condition()
        .map(|c| extract_condition(&*c))
        .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));

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
                .filter_map(|s| extract_statement(&**s))
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
                .filter_map(|s| extract_statement(&**s))
                .collect()
        })
        .unwrap_or_default();

    Statement::If(IfStatement {
        condition,
        then_body,
        else_body,
    })
}

fn extract_evaluate<'input>(ctx: &EvaluateStatementContext<'input>) -> Statement {
    // Extract subjects
    let mut subjects = Vec::new();
    if let Some(sel) = ctx.evaluateSelect() {
        subjects.push(extract_evaluate_subject(&*sel));
    }
    for also in ctx.evaluateAlsoSelect_all() {
        if let Some(sel) = also.evaluateSelect() {
            subjects.push(extract_evaluate_subject(&*sel));
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
                    let text = w.get_text().to_uppercase();
                    let text = text.strip_prefix("WHEN").unwrap_or(&text).trim();
                    if text == "ANY" {
                        WhenValue::Any
                    } else if text == "OTHER" {
                        WhenValue::Any
                    } else {
                        WhenValue::Value(extract_identifier_or_literal_from_text(text))
                    }
                })
                .collect();
            let body: Vec<Statement> = wp
                .statement_all()
                .iter()
                .filter_map(|s| extract_statement(&**s))
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
                .filter_map(|s| extract_statement(&**s))
                .collect()
        })
        .unwrap_or_default();

    Statement::Evaluate(EvaluateStatement {
        subjects,
        when_branches,
        when_other,
    })
}

fn extract_perform<'input>(ctx: &PerformStatementContext<'input>) -> Statement {
    if let Some(inline) = ctx.performInlineStatement() {
        let loop_type = inline
            .performType()
            .map(|pt| extract_perform_type(&*pt))
            .unwrap_or(PerformLoopType::Once);
        let body: Vec<Statement> = inline
            .statement_all()
            .iter()
            .filter_map(|s| extract_statement(&**s))
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
            .map(|pt| extract_perform_type(&*pt))
            .unwrap_or(PerformLoopType::Once);
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

fn extract_goto<'input>(ctx: &GoToStatementContext<'input>) -> Statement {
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
        let text = dep.get_text().to_uppercase();
        let targets: Vec<String> = text
            .split_whitespace()
            .filter(|w| *w != "DEPENDINGON" && *w != "DEPENDING" && *w != "ON")
            .map(|s| s.to_string())
            .collect();
        Statement::GoTo(GoToStatement {
            targets,
            depending: None,
        })
    } else {
        Statement::GoTo(GoToStatement {
            targets: Vec::new(),
            depending: None,
        })
    }
}

fn extract_stop<'input>(ctx: &StopStatementContext<'input>) -> Statement {
    if ctx.RUN().is_some() {
        Statement::StopRun
    } else {
        Statement::StopRun
    }
}

fn extract_initialize<'input>(ctx: &InitializeStatementContext<'input>) -> Statement {
    let targets: Vec<DataReference> = ctx
        .identifier_all()
        .iter()
        .map(|id| extract_data_ref_from_identifier(&**id))
        .collect();

    Statement::Initialize(InitializeStatement {
        targets,
        replacing: Vec::new(),
    })
}

fn extract_call<'input>(ctx: &CallStatementContext<'input>) -> Statement {
    let program = if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&*id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&*lit)
    } else {
        Operand::Literal(Literal::Alphanumeric(String::new()))
    };

    Statement::Call(CallStatement {
        program,
        using: Vec::new(), // Simplified: skip USING extraction for now
        returning: None,
        on_exception: Vec::new(),
        not_on_exception: Vec::new(),
    })
}

fn extract_accept<'input>(ctx: &AcceptStatementContext<'input>) -> Statement {
    let text = ctx.get_text().to_uppercase();
    let name = text
        .strip_prefix("ACCEPT")
        .unwrap_or("")
        .trim()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
    Statement::Accept(AcceptStatement {
        target: make_data_ref(&name),
        from: AcceptSource::Sysin,
    })
}

// ---------------------------------------------------------------------------
// File I/O statement extractors
// ---------------------------------------------------------------------------

fn extract_open<'input>(ctx: &OpenStatementContext<'input>) -> Statement {
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

fn extract_close<'input>(ctx: &CloseStatementContext<'input>) -> Statement {
    let files: Vec<String> = ctx
        .closeFile_all()
        .iter()
        .filter_map(|cf| cf.fileName().map(|f| f.get_text().trim().to_uppercase()))
        .collect();
    Statement::Close(CloseStatement { files })
}

fn extract_read<'input>(ctx: &ReadStatementContext<'input>) -> Statement {
    let file_name = ctx
        .fileName()
        .map(|f| f.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let into = ctx.readInto().and_then(|ri| {
        ri.identifier()
            .map(|id| extract_data_ref_from_identifier(&*id))
    });

    let key = ctx.readKey().and_then(|rk| {
        rk.qualifiedDataName()
            .map(|qdn| extract_data_ref_from_qualified(&*qdn))
    });

    let at_end = ctx
        .atEndPhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(&**s))
                .collect()
        })
        .unwrap_or_default();

    let not_at_end = ctx
        .notAtEndPhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(&**s))
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

fn extract_write<'input>(ctx: &WriteStatementContext<'input>) -> Statement {
    let record_name = ctx
        .recordName()
        .map(|rn| rn.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let from = ctx.writeFromPhrase().and_then(|wfp| {
        if let Some(id) = wfp.identifier() {
            Some(extract_data_ref_from_identifier(&*id))
        } else {
            None
        }
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
                .replace("S", "")
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
                .filter_map(|s| extract_statement(&**s))
                .collect()
        })
        .unwrap_or_default();

    let not_at_eop = ctx
        .writeNotAtEndOfPagePhrase()
        .map(|p| {
            p.statement_all()
                .iter()
                .filter_map(|s| extract_statement(&**s))
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

fn extract_rewrite<'input>(ctx: &RewriteStatementContext<'input>) -> Statement {
    let record_name = ctx
        .recordName()
        .map(|rn| rn.get_text().trim().to_uppercase())
        .unwrap_or_default();

    let from = ctx.rewriteFrom().and_then(|rf| {
        rf.identifier()
            .map(|id| extract_data_ref_from_identifier(&*id))
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

fn extract_delete<'input>(ctx: &DeleteStatementContext<'input>) -> Statement {
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

fn extract_invalid_key_stmts<'input>(
    ctx: Option<&InvalidKeyPhraseContext<'input>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(&**s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_invalid_key_stmts<'input>(
    ctx: Option<&NotInvalidKeyPhraseContext<'input>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(&**s))
            .collect()
    })
    .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Perform type extraction
// ---------------------------------------------------------------------------

fn extract_perform_type<'input>(ctx: &PerformTypeContext<'input>) -> PerformLoopType {
    if let Some(times_ctx) = ctx.performTimes() {
        let count = if let Some(id) = times_ctx.identifier() {
            Operand::DataRef(extract_data_ref_from_identifier(&*id))
        } else if let Some(int_lit) = times_ctx.integerLiteral() {
            Operand::Literal(Literal::Numeric(int_lit.get_text().trim().to_string()))
        } else {
            Operand::Literal(Literal::Numeric("1".to_string()))
        };
        PerformLoopType::Times(count)
    } else if let Some(until_ctx) = ctx.performUntil() {
        let test_before = until_ctx
            .performTestClause()
            .map(|tc| !tc.get_text().to_uppercase().contains("AFTER"))
            .unwrap_or(true);
        let condition = until_ctx
            .condition()
            .map(|c| extract_condition(&*c))
            .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));
        PerformLoopType::Until {
            test_before,
            condition,
        }
    } else if let Some(varying_ctx) = ctx.performVarying() {
        extract_perform_varying(&*varying_ctx)
    } else {
        PerformLoopType::Once
    }
}

fn extract_perform_varying<'input>(
    ctx: &PerformVaryingContext<'input>,
) -> PerformLoopType {
    let test_before = ctx
        .performTestClause()
        .map(|tc| !tc.get_text().to_uppercase().contains("AFTER"))
        .unwrap_or(true);

    // Extract varying clause from text (simplified)
    let text = ctx
        .performVaryingClause()
        .map(|vc| vc.get_text().to_uppercase())
        .unwrap_or_default();

    // Parse: counter FROM start BY increment UNTIL condition
    let parts: Vec<&str> = text.split_whitespace().collect();

    let counter_name = parts.first().unwrap_or(&"I").to_string();
    let from_val = find_keyword_value(&parts, "FROM").unwrap_or("1".to_string());
    let by_val = find_keyword_value(&parts, "BY").unwrap_or("1".to_string());

    // Extract condition from the varying clause context
    let condition = ctx
        .performVaryingClause()
        .and_then(|vc| {
            // The varying clause text includes UNTIL - extract condition after UNTIL
            let vc_text = vc.get_text().to_uppercase();
            if let Some(until_pos) = vc_text.find("UNTIL") {
                let cond_text = vc_text[until_pos + 5..].trim();
                Some(parse_simple_condition(cond_text))
            } else {
                None
            }
        })
        .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));

    PerformLoopType::Varying {
        test_before,
        counter: make_data_ref(&counter_name),
        from: extract_operand_from_identifier_or_literal_ctx(&from_val),
        by: extract_operand_from_identifier_or_literal_ctx(&by_val),
        until: condition,
        after: Vec::new(),
    }
}

/// Find value after a keyword in a token list (e.g., "FROM" -> next token).
fn find_keyword_value(parts: &[&str], keyword: &str) -> Option<String> {
    parts
        .iter()
        .position(|p| *p == keyword)
        .and_then(|pos| parts.get(pos + 1))
        .map(|s| s.to_string())
}

// ---------------------------------------------------------------------------
// Condition extraction
// ---------------------------------------------------------------------------

fn extract_condition<'input>(ctx: &ConditionContext<'input>) -> Condition {
    let base = ctx
        .combinableCondition()
        .map(|cc| extract_combinable_condition(&*cc))
        .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));

    // Process AND/OR chains
    let and_or_list = ctx.andOrCondition_all();
    if and_or_list.is_empty() {
        return base;
    }

    let mut result = base;
    for ao in &and_or_list {
        let right = ao
            .combinableCondition()
            .map(|cc| extract_combinable_condition(&*cc))
            .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));
        if ao.AND().is_some() {
            result = Condition::And(Box::new(result), Box::new(right));
        } else {
            result = Condition::Or(Box::new(result), Box::new(right));
        }
    }
    result
}

fn extract_combinable_condition<'input>(
    ctx: &CombinableConditionContext<'input>,
) -> Condition {
    let negated = ctx.NOT().is_some();
    let cond = ctx
        .simpleCondition()
        .map(|sc| extract_simple_condition(&*sc))
        .unwrap_or(Condition::ConditionName(make_data_ref("TRUE")));

    if negated {
        Condition::Not(Box::new(cond))
    } else {
        cond
    }
}

fn extract_simple_condition<'input>(
    ctx: &SimpleConditionContext<'input>,
) -> Condition {
    // Parenthesized condition
    if ctx.LPARENCHAR().is_some() {
        if let Some(inner) = ctx.condition() {
            return extract_condition(&*inner);
        }
    }

    // Relation condition
    if let Some(rel) = ctx.relationCondition() {
        return extract_relation_condition(&*rel);
    }

    // Class condition
    if let Some(cls) = ctx.classCondition() {
        return extract_class_condition(&*cls);
    }

    // Condition name (88-level)
    if let Some(cnr) = ctx.conditionNameReference() {
        let name = cnr.get_text().trim().to_uppercase();
        return Condition::ConditionName(make_data_ref(&name));
    }

    // Fallback
    Condition::ConditionName(make_data_ref("TRUE"))
}

fn extract_relation_condition<'input>(
    ctx: &RelationConditionContext<'input>,
) -> Condition {
    if let Some(arith_cmp) = ctx.relationArithmeticComparison() {
        let exprs = arith_cmp.arithmeticExpression_all();
        let left = exprs
            .first()
            .map(|e| arith_expr_to_operand(&extract_arith_expr(&**e)))
            .unwrap_or(Operand::Literal(Literal::Numeric("0".to_string())));
        let right = exprs
            .get(1)
            .map(|e| arith_expr_to_operand(&extract_arith_expr(&**e)))
            .unwrap_or(Operand::Literal(Literal::Numeric("0".to_string())));
        let op = arith_cmp
            .relationalOperator()
            .map(|ro| extract_relational_op(&*ro))
            .unwrap_or(ComparisonOp::Equal);
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

fn extract_relational_op<'input>(ctx: &RelationalOperatorContext<'input>) -> ComparisonOp {
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

    if has_not && has_equal {
        ComparisonOp::NotEqual
    } else if has_not && has_greater {
        ComparisonOp::LessOrEqual
    } else if has_not && has_less {
        ComparisonOp::GreaterOrEqual
    } else if has_greater && ctx.OR().is_some() && has_equal {
        ComparisonOp::GreaterOrEqual
    } else if has_less && ctx.OR().is_some() && has_equal {
        ComparisonOp::LessOrEqual
    } else if has_greater {
        ComparisonOp::GreaterThan
    } else if has_less {
        ComparisonOp::LessThan
    } else {
        ComparisonOp::Equal
    }
}

fn extract_class_condition<'input>(ctx: &ClassConditionContext<'input>) -> Condition {
    let field = ctx
        .identifier()
        .map(|id| extract_data_ref_from_identifier(&*id))
        .unwrap_or_else(|| make_data_ref("UNKNOWN"));

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

fn extract_arith_expr<'input>(ctx: &ArithmeticExpressionContext<'input>) -> ArithExpr {
    let base = ctx
        .multDivs()
        .map(|md| extract_mult_divs(&*md))
        .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))));

    let plus_minus_list = ctx.plusMinus_all();
    if plus_minus_list.is_empty() {
        return base;
    }

    let mut result = base;
    for pm in &plus_minus_list {
        let right = pm
            .multDivs()
            .map(|md| extract_mult_divs(&*md))
            .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))));
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

fn extract_mult_divs<'input>(ctx: &MultDivsContext<'input>) -> ArithExpr {
    let base = ctx
        .powers()
        .map(|p| extract_powers(&*p))
        .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))));

    let md_list = ctx.multDiv_all();
    if md_list.is_empty() {
        return base;
    }

    let mut result = base;
    for md in &md_list {
        let right = md
            .powers()
            .map(|p| extract_powers(&*p))
            .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))));
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

fn extract_powers<'input>(ctx: &PowersContext<'input>) -> ArithExpr {
    let has_negate = ctx.MINUSCHAR().is_some();

    let base = ctx
        .basis()
        .map(|b| extract_basis(&*b))
        .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            "0".to_string(),
        ))));

    let power_list = ctx.power_all();
    let mut result = base;
    for pw in &power_list {
        let right = pw
            .basis()
            .map(|b| extract_basis(&*b))
            .unwrap_or(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "0".to_string(),
            ))));
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

fn extract_basis<'input>(ctx: &BasisContext<'input>) -> ArithExpr {
    if let Some(expr) = ctx.arithmeticExpression() {
        ArithExpr::Paren(Box::new(extract_arith_expr(&*expr)))
    } else if let Some(id) = ctx.identifier() {
        ArithExpr::Operand(Operand::DataRef(extract_data_ref_from_identifier(&*id)))
    } else if let Some(lit) = ctx.literal() {
        ArithExpr::Operand(extract_literal_operand(&*lit))
    } else {
        ArithExpr::Operand(Operand::Literal(Literal::Numeric(
            ctx.get_text().trim().to_string(),
        )))
    }
}

// ---------------------------------------------------------------------------
// Helper functions: identifier/literal extraction
// ---------------------------------------------------------------------------

/// Extract a DataReference from an IdentifierContext.
fn extract_data_ref_from_identifier<'input>(
    ctx: &IdentifierContext<'input>,
) -> DataReference {
    if let Some(qdn) = ctx.qualifiedDataName() {
        extract_data_ref_from_qualified(&*qdn)
    } else if let Some(tc) = ctx.tableCall() {
        let base = tc
            .qualifiedDataName()
            .map(|qdn| extract_data_ref_from_qualified(&*qdn))
            .unwrap_or_else(|| make_data_ref(&tc.get_text()));
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
        DataReference {
            subscripts,
            ..base
        }
    } else {
        make_data_ref(&ctx.get_text())
    }
}

/// Extract a DataReference from a QualifiedDataNameContext.
fn extract_data_ref_from_qualified<'input>(
    ctx: &QualifiedDataNameContext<'input>,
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

/// Extract an Operand from a LiteralContext.
fn extract_literal_operand<'input>(ctx: &LiteralContext<'input>) -> Operand {
    if let Some(num) = ctx.numericLiteral() {
        Operand::Literal(Literal::Numeric(num.get_text().trim().to_string()))
    } else if let Some(fig) = ctx.figurativeConstant() {
        let text = fig.get_text().to_uppercase();
        let fc = if text.contains("SPACE") {
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

/// Extract operand from MoveToSendingArea.
fn extract_operand_from_sending_area<'input>(
    ctx: &MoveToSendingAreaContext<'input>,
) -> Operand {
    if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&*id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&*lit)
    } else {
        extract_identifier_or_literal_from_text(&ctx.get_text())
    }
}

/// Extract operand from AddFrom context.
fn extract_operand_from_add_from<'input>(ctx: &AddFromContext<'input>) -> Operand {
    if let Some(id) = ctx.identifier() {
        Operand::DataRef(extract_data_ref_from_identifier(&*id))
    } else if let Some(lit) = ctx.literal() {
        extract_literal_operand(&*lit)
    } else {
        extract_identifier_or_literal_from_text(&ctx.get_text())
    }
}

/// Extract evaluate subject from EvaluateSelectContext.
fn extract_evaluate_subject<'input>(
    ctx: &EvaluateSelectContext<'input>,
) -> EvaluateSubject {
    let text = ctx.get_text().to_uppercase();
    if text.trim() == "TRUE" {
        return EvaluateSubject::Bool(true);
    }
    if text.trim() == "FALSE" {
        return EvaluateSubject::Bool(false);
    }

    if let Some(id) = ctx.identifier() {
        EvaluateSubject::Expr(Operand::DataRef(extract_data_ref_from_identifier(&*id)))
    } else if let Some(lit) = ctx.literal() {
        EvaluateSubject::Expr(extract_literal_operand(&*lit))
    } else {
        EvaluateSubject::Expr(extract_identifier_or_literal_from_text(&text))
    }
}

// ---------------------------------------------------------------------------
// Size error phrase extraction
// ---------------------------------------------------------------------------

fn extract_size_error_stmts<'input>(
    ctx: Option<&OnSizeErrorPhraseContext<'input>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(&**s))
            .collect()
    })
    .unwrap_or_default()
}

fn extract_not_size_error_stmts<'input>(
    ctx: Option<&NotOnSizeErrorPhraseContext<'input>>,
) -> Vec<Statement> {
    ctx.map(|c| {
        c.statement_all()
            .iter()
            .filter_map(|s| extract_statement(&**s))
            .collect()
    })
    .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Text-based helper functions
// ---------------------------------------------------------------------------

/// Create a simple DataReference from a name string.
fn make_data_ref(name: &str) -> DataReference {
    let clean = name.trim().to_uppercase();
    // Handle qualified names (X IN Y)
    let parts: Vec<&str> = clean.split(|c| c == ' ').collect();
    let (data_name, qualifiers) = if parts.len() >= 3 {
        let mut quals = Vec::new();
        let mut i = 2;
        while i < parts.len() {
            if parts.get(i.wrapping_sub(1)).map_or(false, |p| *p == "IN" || *p == "OF") {
                quals.push(parts[i].to_string());
            }
            i += 1;
        }
        (parts[0].to_string(), quals)
    } else {
        (clean.clone(), Vec::new())
    };

    DataReference {
        name: data_name,
        qualifiers,
        subscripts: Vec::new(),
        ref_mod: None,
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

/// Extract giving phrase targets from text.
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
        let rounded = tokens.get(i + 1).map_or(false, |t| *t == "ROUNDED");
        targets.push(ArithTarget {
            field: make_data_ref(name),
            rounded,
        });
        i += if rounded { 2 } else { 1 };
    }
    targets
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

/// Convert an ArithExpr to an Operand (for simple expressions).
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
