#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/cobol2rust/grammar/Cobol85Preprocessor.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::cobol85preprocessorparser::*;

pub trait Cobol85PreprocessorListener<'input> : ParseTreeListener<'input,Cobol85PreprocessorParserContextType>{
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#startRule}.
 * @param ctx the parse tree
 */
fn enter_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#startRule}.
 * @param ctx the parse tree
 */
fn exit_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#compilerOptions}.
 * @param ctx the parse tree
 */
fn enter_compilerOptions(&mut self, _ctx: &CompilerOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOptions}.
 * @param ctx the parse tree
 */
fn exit_compilerOptions(&mut self, _ctx: &CompilerOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#compilerXOpts}.
 * @param ctx the parse tree
 */
fn enter_compilerXOpts(&mut self, _ctx: &CompilerXOptsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#compilerXOpts}.
 * @param ctx the parse tree
 */
fn exit_compilerXOpts(&mut self, _ctx: &CompilerXOptsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#compilerOption}.
 * @param ctx the parse tree
 */
fn enter_compilerOption(&mut self, _ctx: &CompilerOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOption}.
 * @param ctx the parse tree
 */
fn exit_compilerOption(&mut self, _ctx: &CompilerOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#execCicsStatement}.
 * @param ctx the parse tree
 */
fn enter_execCicsStatement(&mut self, _ctx: &ExecCicsStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#execCicsStatement}.
 * @param ctx the parse tree
 */
fn exit_execCicsStatement(&mut self, _ctx: &ExecCicsStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#execSqlStatement}.
 * @param ctx the parse tree
 */
fn enter_execSqlStatement(&mut self, _ctx: &ExecSqlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlStatement}.
 * @param ctx the parse tree
 */
fn exit_execSqlStatement(&mut self, _ctx: &ExecSqlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#execSqlImsStatement}.
 * @param ctx the parse tree
 */
fn enter_execSqlImsStatement(&mut self, _ctx: &ExecSqlImsStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlImsStatement}.
 * @param ctx the parse tree
 */
fn exit_execSqlImsStatement(&mut self, _ctx: &ExecSqlImsStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#copyStatement}.
 * @param ctx the parse tree
 */
fn enter_copyStatement(&mut self, _ctx: &CopyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#copyStatement}.
 * @param ctx the parse tree
 */
fn exit_copyStatement(&mut self, _ctx: &CopyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#copySource}.
 * @param ctx the parse tree
 */
fn enter_copySource(&mut self, _ctx: &CopySourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#copySource}.
 * @param ctx the parse tree
 */
fn exit_copySource(&mut self, _ctx: &CopySourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#copyLibrary}.
 * @param ctx the parse tree
 */
fn enter_copyLibrary(&mut self, _ctx: &CopyLibraryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#copyLibrary}.
 * @param ctx the parse tree
 */
fn exit_copyLibrary(&mut self, _ctx: &CopyLibraryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replacingPhrase}.
 * @param ctx the parse tree
 */
fn enter_replacingPhrase(&mut self, _ctx: &ReplacingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replacingPhrase}.
 * @param ctx the parse tree
 */
fn exit_replacingPhrase(&mut self, _ctx: &ReplacingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replaceArea}.
 * @param ctx the parse tree
 */
fn enter_replaceArea(&mut self, _ctx: &ReplaceAreaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replaceArea}.
 * @param ctx the parse tree
 */
fn exit_replaceArea(&mut self, _ctx: &ReplaceAreaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replaceByStatement}.
 * @param ctx the parse tree
 */
fn enter_replaceByStatement(&mut self, _ctx: &ReplaceByStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replaceByStatement}.
 * @param ctx the parse tree
 */
fn exit_replaceByStatement(&mut self, _ctx: &ReplaceByStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replaceOffStatement}.
 * @param ctx the parse tree
 */
fn enter_replaceOffStatement(&mut self, _ctx: &ReplaceOffStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replaceOffStatement}.
 * @param ctx the parse tree
 */
fn exit_replaceOffStatement(&mut self, _ctx: &ReplaceOffStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replaceClause}.
 * @param ctx the parse tree
 */
fn enter_replaceClause(&mut self, _ctx: &ReplaceClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replaceClause}.
 * @param ctx the parse tree
 */
fn exit_replaceClause(&mut self, _ctx: &ReplaceClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#directoryPhrase}.
 * @param ctx the parse tree
 */
fn enter_directoryPhrase(&mut self, _ctx: &DirectoryPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#directoryPhrase}.
 * @param ctx the parse tree
 */
fn exit_directoryPhrase(&mut self, _ctx: &DirectoryPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#familyPhrase}.
 * @param ctx the parse tree
 */
fn enter_familyPhrase(&mut self, _ctx: &FamilyPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#familyPhrase}.
 * @param ctx the parse tree
 */
fn exit_familyPhrase(&mut self, _ctx: &FamilyPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replaceable}.
 * @param ctx the parse tree
 */
fn enter_replaceable(&mut self, _ctx: &ReplaceableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replaceable}.
 * @param ctx the parse tree
 */
fn exit_replaceable(&mut self, _ctx: &ReplaceableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#replacement}.
 * @param ctx the parse tree
 */
fn enter_replacement(&mut self, _ctx: &ReplacementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#replacement}.
 * @param ctx the parse tree
 */
fn exit_replacement(&mut self, _ctx: &ReplacementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#ejectStatement}.
 * @param ctx the parse tree
 */
fn enter_ejectStatement(&mut self, _ctx: &EjectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#ejectStatement}.
 * @param ctx the parse tree
 */
fn exit_ejectStatement(&mut self, _ctx: &EjectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#skipStatement}.
 * @param ctx the parse tree
 */
fn enter_skipStatement(&mut self, _ctx: &SkipStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#skipStatement}.
 * @param ctx the parse tree
 */
fn exit_skipStatement(&mut self, _ctx: &SkipStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#titleStatement}.
 * @param ctx the parse tree
 */
fn enter_titleStatement(&mut self, _ctx: &TitleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#titleStatement}.
 * @param ctx the parse tree
 */
fn exit_titleStatement(&mut self, _ctx: &TitleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#pseudoText}.
 * @param ctx the parse tree
 */
fn enter_pseudoText(&mut self, _ctx: &PseudoTextContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#pseudoText}.
 * @param ctx the parse tree
 */
fn exit_pseudoText(&mut self, _ctx: &PseudoTextContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#charData}.
 * @param ctx the parse tree
 */
fn enter_charData(&mut self, _ctx: &CharDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#charData}.
 * @param ctx the parse tree
 */
fn exit_charData(&mut self, _ctx: &CharDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#charDataSql}.
 * @param ctx the parse tree
 */
fn enter_charDataSql(&mut self, _ctx: &CharDataSqlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#charDataSql}.
 * @param ctx the parse tree
 */
fn exit_charDataSql(&mut self, _ctx: &CharDataSqlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#charDataLine}.
 * @param ctx the parse tree
 */
fn enter_charDataLine(&mut self, _ctx: &CharDataLineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#charDataLine}.
 * @param ctx the parse tree
 */
fn exit_charDataLine(&mut self, _ctx: &CharDataLineContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#cobolWord}.
 * @param ctx the parse tree
 */
fn enter_cobolWord(&mut self, _ctx: &CobolWordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#cobolWord}.
 * @param ctx the parse tree
 */
fn exit_cobolWord(&mut self, _ctx: &CobolWordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#filename}.
 * @param ctx the parse tree
 */
fn enter_filename(&mut self, _ctx: &FilenameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#filename}.
 * @param ctx the parse tree
 */
fn exit_filename(&mut self, _ctx: &FilenameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85PreprocessorParser#charDataKeyword}.
 * @param ctx the parse tree
 */
fn enter_charDataKeyword(&mut self, _ctx: &CharDataKeywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85PreprocessorParser#charDataKeyword}.
 * @param ctx the parse tree
 */
fn exit_charDataKeyword(&mut self, _ctx: &CharDataKeywordContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : Cobol85PreprocessorListener<'input> }


