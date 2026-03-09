#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/cobol2rust/grammar/Cobol85Preprocessor.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::cobol85preprocessorparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Cobol85PreprocessorParser}.
 */
pub trait Cobol85PreprocessorVisitor<'input>: ParseTreeVisitor<'input,Cobol85PreprocessorParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#startRule}.
	 * @param ctx the parse tree
	 */
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_compilerOptions(&mut self, ctx: &CompilerOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerXOpts}.
	 * @param ctx the parse tree
	 */
	fn visit_compilerXOpts(&mut self, ctx: &CompilerXOptsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOption}.
	 * @param ctx the parse tree
	 */
	fn visit_compilerOption(&mut self, ctx: &CompilerOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execCicsStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlImsStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_copyStatement(&mut self, ctx: &CopyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copySource}.
	 * @param ctx the parse tree
	 */
	fn visit_copySource(&mut self, ctx: &CopySourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copyLibrary}.
	 * @param ctx the parse tree
	 */
	fn visit_copyLibrary(&mut self, ctx: &CopyLibraryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replacingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_replacingPhrase(&mut self, ctx: &ReplacingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceArea}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceArea(&mut self, ctx: &ReplaceAreaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceByStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceByStatement(&mut self, ctx: &ReplaceByStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceOffStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceOffStatement(&mut self, ctx: &ReplaceOffStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceClause}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceClause(&mut self, ctx: &ReplaceClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#directoryPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_directoryPhrase(&mut self, ctx: &DirectoryPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#familyPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_familyPhrase(&mut self, ctx: &FamilyPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceable}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceable(&mut self, ctx: &ReplaceableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replacement}.
	 * @param ctx the parse tree
	 */
	fn visit_replacement(&mut self, ctx: &ReplacementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#ejectStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ejectStatement(&mut self, ctx: &EjectStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#skipStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_skipStatement(&mut self, ctx: &SkipStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#titleStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_titleStatement(&mut self, ctx: &TitleStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#pseudoText}.
	 * @param ctx the parse tree
	 */
	fn visit_pseudoText(&mut self, ctx: &PseudoTextContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charData}.
	 * @param ctx the parse tree
	 */
	fn visit_charData(&mut self, ctx: &CharDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataSql}.
	 * @param ctx the parse tree
	 */
	fn visit_charDataSql(&mut self, ctx: &CharDataSqlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataLine}.
	 * @param ctx the parse tree
	 */
	fn visit_charDataLine(&mut self, ctx: &CharDataLineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#cobolWord}.
	 * @param ctx the parse tree
	 */
	fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#filename}.
	 * @param ctx the parse tree
	 */
	fn visit_filename(&mut self, ctx: &FilenameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataKeyword}.
	 * @param ctx the parse tree
	 */
	fn visit_charDataKeyword(&mut self, ctx: &CharDataKeywordContext<'input>) { self.visit_children(ctx) }

}

pub trait Cobol85PreprocessorVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= Cobol85PreprocessorParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#startRule}.
	 * @param ctx the parse tree
	 */
		fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_compilerOptions(&mut self, ctx: &CompilerOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerXOpts}.
	 * @param ctx the parse tree
	 */
		fn visit_compilerXOpts(&mut self, ctx: &CompilerXOptsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#compilerOption}.
	 * @param ctx the parse tree
	 */
		fn visit_compilerOption(&mut self, ctx: &CompilerOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execCicsStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#execSqlImsStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copyStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_copyStatement(&mut self, ctx: &CopyStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copySource}.
	 * @param ctx the parse tree
	 */
		fn visit_copySource(&mut self, ctx: &CopySourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#copyLibrary}.
	 * @param ctx the parse tree
	 */
		fn visit_copyLibrary(&mut self, ctx: &CopyLibraryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replacingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_replacingPhrase(&mut self, ctx: &ReplacingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceArea}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceArea(&mut self, ctx: &ReplaceAreaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceByStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceByStatement(&mut self, ctx: &ReplaceByStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceOffStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceOffStatement(&mut self, ctx: &ReplaceOffStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceClause}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceClause(&mut self, ctx: &ReplaceClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#directoryPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_directoryPhrase(&mut self, ctx: &DirectoryPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#familyPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_familyPhrase(&mut self, ctx: &FamilyPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replaceable}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceable(&mut self, ctx: &ReplaceableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#replacement}.
	 * @param ctx the parse tree
	 */
		fn visit_replacement(&mut self, ctx: &ReplacementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#ejectStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ejectStatement(&mut self, ctx: &EjectStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#skipStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_skipStatement(&mut self, ctx: &SkipStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#titleStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_titleStatement(&mut self, ctx: &TitleStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#pseudoText}.
	 * @param ctx the parse tree
	 */
		fn visit_pseudoText(&mut self, ctx: &PseudoTextContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charData}.
	 * @param ctx the parse tree
	 */
		fn visit_charData(&mut self, ctx: &CharDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataSql}.
	 * @param ctx the parse tree
	 */
		fn visit_charDataSql(&mut self, ctx: &CharDataSqlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataLine}.
	 * @param ctx the parse tree
	 */
		fn visit_charDataLine(&mut self, ctx: &CharDataLineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#cobolWord}.
	 * @param ctx the parse tree
	 */
		fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#filename}.
	 * @param ctx the parse tree
	 */
		fn visit_filename(&mut self, ctx: &FilenameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85PreprocessorParser#charDataKeyword}.
	 * @param ctx the parse tree
	 */
		fn visit_charDataKeyword(&mut self, ctx: &CharDataKeywordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> Cobol85PreprocessorVisitor<'input> for T
where
	T: Cobol85PreprocessorVisitorCompat<'input>
{
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_startRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compilerOptions(&mut self, ctx: &CompilerOptionsContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_compilerOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compilerXOpts(&mut self, ctx: &CompilerXOptsContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_compilerXOpts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compilerOption(&mut self, ctx: &CompilerOptionContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_compilerOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_execCicsStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_execSqlStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_execSqlImsStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_copyStatement(&mut self, ctx: &CopyStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_copyStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_copySource(&mut self, ctx: &CopySourceContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_copySource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_copyLibrary(&mut self, ctx: &CopyLibraryContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_copyLibrary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replacingPhrase(&mut self, ctx: &ReplacingPhraseContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replacingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceArea(&mut self, ctx: &ReplaceAreaContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replaceArea(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceByStatement(&mut self, ctx: &ReplaceByStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replaceByStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceOffStatement(&mut self, ctx: &ReplaceOffStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replaceOffStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceClause(&mut self, ctx: &ReplaceClauseContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replaceClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_directoryPhrase(&mut self, ctx: &DirectoryPhraseContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_directoryPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_familyPhrase(&mut self, ctx: &FamilyPhraseContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_familyPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceable(&mut self, ctx: &ReplaceableContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replaceable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replacement(&mut self, ctx: &ReplacementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_replacement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ejectStatement(&mut self, ctx: &EjectStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_ejectStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_skipStatement(&mut self, ctx: &SkipStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_skipStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_titleStatement(&mut self, ctx: &TitleStatementContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_titleStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pseudoText(&mut self, ctx: &PseudoTextContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_pseudoText(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_charData(&mut self, ctx: &CharDataContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_charData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_charDataSql(&mut self, ctx: &CharDataSqlContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_charDataSql(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_charDataLine(&mut self, ctx: &CharDataLineContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_charDataLine(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_cobolWord(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filename(&mut self, ctx: &FilenameContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_filename(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_charDataKeyword(&mut self, ctx: &CharDataKeywordContext<'input>){
		let result = <Self as Cobol85PreprocessorVisitorCompat>::visit_charDataKeyword(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}