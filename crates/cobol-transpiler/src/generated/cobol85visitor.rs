#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/cobol2rust/grammar/Cobol85.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::cobol85parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Cobol85Parser}.
 */
pub trait Cobol85Visitor<'input>: ParseTreeVisitor<'input,Cobol85ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startRule}.
	 * @param ctx the parse tree
	 */
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_programUnit(&mut self, ctx: &ProgramUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#endProgramStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_endProgramStatement(&mut self, ctx: &EndProgramStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identificationDivision}.
	 * @param ctx the parse tree
	 */
	fn visit_identificationDivision(&mut self, ctx: &IdentificationDivisionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identificationDivisionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_identificationDivisionBody(&mut self, ctx: &IdentificationDivisionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programIdParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_programIdParagraph(&mut self, ctx: &ProgramIdParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#authorParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_authorParagraph(&mut self, ctx: &AuthorParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#installationParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_installationParagraph(&mut self, ctx: &InstallationParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dateWrittenParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_dateWrittenParagraph(&mut self, ctx: &DateWrittenParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dateCompiledParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_dateCompiledParagraph(&mut self, ctx: &DateCompiledParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#securityParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_securityParagraph(&mut self, ctx: &SecurityParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#remarksParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_remarksParagraph(&mut self, ctx: &RemarksParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentDivision}.
	 * @param ctx the parse tree
	 */
	fn visit_environmentDivision(&mut self, ctx: &EnvironmentDivisionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentDivisionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_environmentDivisionBody(&mut self, ctx: &EnvironmentDivisionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#configurationSection}.
	 * @param ctx the parse tree
	 */
	fn visit_configurationSection(&mut self, ctx: &ConfigurationSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#configurationSectionParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_configurationSectionParagraph(&mut self, ctx: &ConfigurationSectionParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sourceComputerParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_sourceComputerParagraph(&mut self, ctx: &SourceComputerParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#objectComputerParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_objectComputerParagraph(&mut self, ctx: &ObjectComputerParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#objectComputerClause}.
	 * @param ctx the parse tree
	 */
	fn visit_objectComputerClause(&mut self, ctx: &ObjectComputerClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#memorySizeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_memorySizeClause(&mut self, ctx: &MemorySizeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#diskSizeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_diskSizeClause(&mut self, ctx: &DiskSizeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClause}.
	 * @param ctx the parse tree
	 */
	fn visit_collatingSequenceClause(&mut self, ctx: &CollatingSequenceClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseAlphanumeric}.
	 * @param ctx the parse tree
	 */
	fn visit_collatingSequenceClauseAlphanumeric(&mut self, ctx: &CollatingSequenceClauseAlphanumericContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseNational}.
	 * @param ctx the parse tree
	 */
	fn visit_collatingSequenceClauseNational(&mut self, ctx: &CollatingSequenceClauseNationalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#segmentLimitClause}.
	 * @param ctx the parse tree
	 */
	fn visit_segmentLimitClause(&mut self, ctx: &SegmentLimitClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#characterSetClause}.
	 * @param ctx the parse tree
	 */
	fn visit_characterSetClause(&mut self, ctx: &CharacterSetClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialNamesParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_specialNamesParagraph(&mut self, ctx: &SpecialNamesParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialNameClause}.
	 * @param ctx the parse tree
	 */
	fn visit_specialNameClause(&mut self, ctx: &SpecialNameClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClause}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetClause(&mut self, ctx: &AlphabetClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetClauseFormat1(&mut self, ctx: &AlphabetClauseFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetLiterals}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetLiterals(&mut self, ctx: &AlphabetLiteralsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetThrough(&mut self, ctx: &AlphabetThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetAlso}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetAlso(&mut self, ctx: &AlphabetAlsoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetClauseFormat2(&mut self, ctx: &AlphabetClauseFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#channelClause}.
	 * @param ctx the parse tree
	 */
	fn visit_channelClause(&mut self, ctx: &ChannelClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClause}.
	 * @param ctx the parse tree
	 */
	fn visit_classClause(&mut self, ctx: &ClassClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_classClauseThrough(&mut self, ctx: &ClassClauseThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_classClauseFrom(&mut self, ctx: &ClassClauseFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseTo}.
	 * @param ctx the parse tree
	 */
	fn visit_classClauseTo(&mut self, ctx: &ClassClauseToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#currencySignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_currencySignClause(&mut self, ctx: &CurrencySignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#decimalPointClause}.
	 * @param ctx the parse tree
	 */
	fn visit_decimalPointClause(&mut self, ctx: &DecimalPointClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#defaultComputationalSignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultComputationalSignClause(&mut self, ctx: &DefaultComputationalSignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#defaultDisplaySignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultDisplaySignClause(&mut self, ctx: &DefaultDisplaySignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameClause}.
	 * @param ctx the parse tree
	 */
	fn visit_environmentSwitchNameClause(&mut self, ctx: &EnvironmentSwitchNameClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameSpecialNamesStatusPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_environmentSwitchNameSpecialNamesStatusPhrase(&mut self, ctx: &EnvironmentSwitchNameSpecialNamesStatusPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#odtClause}.
	 * @param ctx the parse tree
	 */
	fn visit_odtClause(&mut self, ctx: &OdtClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reserveNetworkClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reserveNetworkClause(&mut self, ctx: &ReserveNetworkClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharactersClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicCharactersClause(&mut self, ctx: &SymbolicCharactersClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharacters}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicCharacters(&mut self, ctx: &SymbolicCharactersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inputOutputSection}.
	 * @param ctx the parse tree
	 */
	fn visit_inputOutputSection(&mut self, ctx: &InputOutputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inputOutputSectionParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_inputOutputSectionParagraph(&mut self, ctx: &InputOutputSectionParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_fileControlParagraph(&mut self, ctx: &FileControlParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_fileControlEntry(&mut self, ctx: &FileControlEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#selectClause}.
	 * @param ctx the parse tree
	 */
	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlClause}.
	 * @param ctx the parse tree
	 */
	fn visit_fileControlClause(&mut self, ctx: &FileControlClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#assignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_assignClause(&mut self, ctx: &AssignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reserveClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reserveClause(&mut self, ctx: &ReserveClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#organizationClause}.
	 * @param ctx the parse tree
	 */
	fn visit_organizationClause(&mut self, ctx: &OrganizationClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paddingCharacterClause}.
	 * @param ctx the parse tree
	 */
	fn visit_paddingCharacterClause(&mut self, ctx: &PaddingCharacterClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordDelimiterClause}.
	 * @param ctx the parse tree
	 */
	fn visit_recordDelimiterClause(&mut self, ctx: &RecordDelimiterClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#accessModeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_accessModeClause(&mut self, ctx: &AccessModeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_recordKeyClause(&mut self, ctx: &RecordKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alternateRecordKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_alternateRecordKeyClause(&mut self, ctx: &AlternateRecordKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#passwordClause}.
	 * @param ctx the parse tree
	 */
	fn visit_passwordClause(&mut self, ctx: &PasswordClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileStatusClause}.
	 * @param ctx the parse tree
	 */
	fn visit_fileStatusClause(&mut self, ctx: &FileStatusClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relativeKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_relativeKeyClause(&mut self, ctx: &RelativeKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ioControlParagraph}.
	 * @param ctx the parse tree
	 */
	fn visit_ioControlParagraph(&mut self, ctx: &IoControlParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ioControlClause}.
	 * @param ctx the parse tree
	 */
	fn visit_ioControlClause(&mut self, ctx: &IoControlClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunClause}.
	 * @param ctx the parse tree
	 */
	fn visit_rerunClause(&mut self, ctx: &RerunClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryRecords}.
	 * @param ctx the parse tree
	 */
	fn visit_rerunEveryRecords(&mut self, ctx: &RerunEveryRecordsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryOf}.
	 * @param ctx the parse tree
	 */
	fn visit_rerunEveryOf(&mut self, ctx: &RerunEveryOfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryClock}.
	 * @param ctx the parse tree
	 */
	fn visit_rerunEveryClock(&mut self, ctx: &RerunEveryClockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sameClause}.
	 * @param ctx the parse tree
	 */
	fn visit_sameClause(&mut self, ctx: &SameClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multipleFileClause}.
	 * @param ctx the parse tree
	 */
	fn visit_multipleFileClause(&mut self, ctx: &MultipleFileClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multipleFilePosition}.
	 * @param ctx the parse tree
	 */
	fn visit_multipleFilePosition(&mut self, ctx: &MultipleFilePositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#commitmentControlClause}.
	 * @param ctx the parse tree
	 */
	fn visit_commitmentControlClause(&mut self, ctx: &CommitmentControlClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDivision}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDivision(&mut self, ctx: &DataDivisionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDivisionSection}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDivisionSection(&mut self, ctx: &DataDivisionSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileSection}.
	 * @param ctx the parse tree
	 */
	fn visit_fileSection(&mut self, ctx: &FileSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_fileDescriptionEntry(&mut self, ctx: &FileDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntryClause}.
	 * @param ctx the parse tree
	 */
	fn visit_fileDescriptionEntryClause(&mut self, ctx: &FileDescriptionEntryClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#externalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_externalClause(&mut self, ctx: &ExternalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#globalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_globalClause(&mut self, ctx: &GlobalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#blockContainsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_blockContainsClause(&mut self, ctx: &BlockContainsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#blockContainsTo}.
	 * @param ctx the parse tree
	 */
	fn visit_blockContainsTo(&mut self, ctx: &BlockContainsToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_recordContainsClause(&mut self, ctx: &RecordContainsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_recordContainsClauseFormat1(&mut self, ctx: &RecordContainsClauseFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_recordContainsClauseFormat2(&mut self, ctx: &RecordContainsClauseFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat3}.
	 * @param ctx the parse tree
	 */
	fn visit_recordContainsClauseFormat3(&mut self, ctx: &RecordContainsClauseFormat3Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsTo}.
	 * @param ctx the parse tree
	 */
	fn visit_recordContainsTo(&mut self, ctx: &RecordContainsToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#labelRecordsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_labelRecordsClause(&mut self, ctx: &LabelRecordsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#valueOfClause}.
	 * @param ctx the parse tree
	 */
	fn visit_valueOfClause(&mut self, ctx: &ValueOfClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#valuePair}.
	 * @param ctx the parse tree
	 */
	fn visit_valuePair(&mut self, ctx: &ValuePairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRecordsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataRecordsClause(&mut self, ctx: &DataRecordsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageClause}.
	 * @param ctx the parse tree
	 */
	fn visit_linageClause(&mut self, ctx: &LinageClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageAt}.
	 * @param ctx the parse tree
	 */
	fn visit_linageAt(&mut self, ctx: &LinageAtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageFootingAt}.
	 * @param ctx the parse tree
	 */
	fn visit_linageFootingAt(&mut self, ctx: &LinageFootingAtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageLinesAtTop}.
	 * @param ctx the parse tree
	 */
	fn visit_linageLinesAtTop(&mut self, ctx: &LinageLinesAtTopContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageLinesAtBottom}.
	 * @param ctx the parse tree
	 */
	fn visit_linageLinesAtBottom(&mut self, ctx: &LinageLinesAtBottomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordingModeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_recordingModeClause(&mut self, ctx: &RecordingModeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#modeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_modeStatement(&mut self, ctx: &ModeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#codeSetClause}.
	 * @param ctx the parse tree
	 */
	fn visit_codeSetClause(&mut self, ctx: &CodeSetClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportClause(&mut self, ctx: &ReportClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBaseSection}.
	 * @param ctx the parse tree
	 */
	fn visit_dataBaseSection(&mut self, ctx: &DataBaseSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBaseSectionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_dataBaseSectionEntry(&mut self, ctx: &DataBaseSectionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#workingStorageSection}.
	 * @param ctx the parse tree
	 */
	fn visit_workingStorageSection(&mut self, ctx: &WorkingStorageSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linkageSection}.
	 * @param ctx the parse tree
	 */
	fn visit_linkageSection(&mut self, ctx: &LinkageSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationSection}.
	 * @param ctx the parse tree
	 */
	fn visit_communicationSection(&mut self, ctx: &CommunicationSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_communicationDescriptionEntry(&mut self, ctx: &CommunicationDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_communicationDescriptionEntryFormat1(&mut self, ctx: &CommunicationDescriptionEntryFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_communicationDescriptionEntryFormat2(&mut self, ctx: &CommunicationDescriptionEntryFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
	fn visit_communicationDescriptionEntryFormat3(&mut self, ctx: &CommunicationDescriptionEntryFormat3Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#destinationCountClause}.
	 * @param ctx the parse tree
	 */
	fn visit_destinationCountClause(&mut self, ctx: &DestinationCountClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#destinationTableClause}.
	 * @param ctx the parse tree
	 */
	fn visit_destinationTableClause(&mut self, ctx: &DestinationTableClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#endKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_endKeyClause(&mut self, ctx: &EndKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#errorKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_errorKeyClause(&mut self, ctx: &ErrorKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageCountClause}.
	 * @param ctx the parse tree
	 */
	fn visit_messageCountClause(&mut self, ctx: &MessageCountClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageDateClause}.
	 * @param ctx the parse tree
	 */
	fn visit_messageDateClause(&mut self, ctx: &MessageDateClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageTimeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_messageTimeClause(&mut self, ctx: &MessageTimeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#statusKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_statusKeyClause(&mut self, ctx: &StatusKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicDestinationClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicDestinationClause(&mut self, ctx: &SymbolicDestinationClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicQueueClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicQueueClause(&mut self, ctx: &SymbolicQueueClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicSourceClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicSourceClause(&mut self, ctx: &SymbolicSourceClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicTerminalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicTerminalClause(&mut self, ctx: &SymbolicTerminalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicSubQueueClause}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicSubQueueClause(&mut self, ctx: &SymbolicSubQueueClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#textLengthClause}.
	 * @param ctx the parse tree
	 */
	fn visit_textLengthClause(&mut self, ctx: &TextLengthClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#localStorageSection}.
	 * @param ctx the parse tree
	 */
	fn visit_localStorageSection(&mut self, ctx: &LocalStorageSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenSection}.
	 * @param ctx the parse tree
	 */
	fn visit_screenSection(&mut self, ctx: &ScreenSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionEntry(&mut self, ctx: &ScreenDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionBlankClause(&mut self, ctx: &ScreenDescriptionBlankClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBellClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionBellClause(&mut self, ctx: &ScreenDescriptionBellClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlinkClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionBlinkClause(&mut self, ctx: &ScreenDescriptionBlinkClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionEraseClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionEraseClause(&mut self, ctx: &ScreenDescriptionEraseClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionLightClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionLightClause(&mut self, ctx: &ScreenDescriptionLightClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionGridClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionGridClause(&mut self, ctx: &ScreenDescriptionGridClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionReverseVideoClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionReverseVideoClause(&mut self, ctx: &ScreenDescriptionReverseVideoClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUnderlineClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionUnderlineClause(&mut self, ctx: &ScreenDescriptionUnderlineClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSizeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionSizeClause(&mut self, ctx: &ScreenDescriptionSizeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionLineClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionLineClause(&mut self, ctx: &ScreenDescriptionLineClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionColumnClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionColumnClause(&mut self, ctx: &ScreenDescriptionColumnClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionForegroundColorClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionForegroundColorClause(&mut self, ctx: &ScreenDescriptionForegroundColorClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBackgroundColorClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionBackgroundColorClause(&mut self, ctx: &ScreenDescriptionBackgroundColorClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionControlClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionControlClause(&mut self, ctx: &ScreenDescriptionControlClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionValueClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionValueClause(&mut self, ctx: &ScreenDescriptionValueClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPictureClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionPictureClause(&mut self, ctx: &ScreenDescriptionPictureClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionFromClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionFromClause(&mut self, ctx: &ScreenDescriptionFromClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionToClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionToClause(&mut self, ctx: &ScreenDescriptionToClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionUsingClause(&mut self, ctx: &ScreenDescriptionUsingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsageClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionUsageClause(&mut self, ctx: &ScreenDescriptionUsageClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionBlankWhenZeroClause(&mut self, ctx: &ScreenDescriptionBlankWhenZeroClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionJustifiedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionJustifiedClause(&mut self, ctx: &ScreenDescriptionJustifiedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionSignClause(&mut self, ctx: &ScreenDescriptionSignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionAutoClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionAutoClause(&mut self, ctx: &ScreenDescriptionAutoClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSecureClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionSecureClause(&mut self, ctx: &ScreenDescriptionSecureClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionRequiredClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionRequiredClause(&mut self, ctx: &ScreenDescriptionRequiredClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionPromptClause(&mut self, ctx: &ScreenDescriptionPromptClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptOccursClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionPromptOccursClause(&mut self, ctx: &ScreenDescriptionPromptOccursClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionFullClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionFullClause(&mut self, ctx: &ScreenDescriptionFullClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionZeroFillClause}.
	 * @param ctx the parse tree
	 */
	fn visit_screenDescriptionZeroFillClause(&mut self, ctx: &ScreenDescriptionZeroFillClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportSection}.
	 * @param ctx the parse tree
	 */
	fn visit_reportSection(&mut self, ctx: &ReportSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescription}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescription(&mut self, ctx: &ReportDescriptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionEntry(&mut self, ctx: &ReportDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionGlobalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionGlobalClause(&mut self, ctx: &ReportDescriptionGlobalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionPageLimitClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionPageLimitClause(&mut self, ctx: &ReportDescriptionPageLimitClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionHeadingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionHeadingClause(&mut self, ctx: &ReportDescriptionHeadingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionFirstDetailClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionFirstDetailClause(&mut self, ctx: &ReportDescriptionFirstDetailClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionLastDetailClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionLastDetailClause(&mut self, ctx: &ReportDescriptionLastDetailClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionFootingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportDescriptionFootingClause(&mut self, ctx: &ReportDescriptionFootingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupDescriptionEntry(&mut self, ctx: &ReportGroupDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupDescriptionEntryFormat1(&mut self, ctx: &ReportGroupDescriptionEntryFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupDescriptionEntryFormat2(&mut self, ctx: &ReportGroupDescriptionEntryFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupDescriptionEntryFormat3(&mut self, ctx: &ReportGroupDescriptionEntryFormat3Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupBlankWhenZeroClause(&mut self, ctx: &ReportGroupBlankWhenZeroClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupColumnNumberClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupColumnNumberClause(&mut self, ctx: &ReportGroupColumnNumberClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupIndicateClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupIndicateClause(&mut self, ctx: &ReportGroupIndicateClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupJustifiedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupJustifiedClause(&mut self, ctx: &ReportGroupJustifiedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupLineNumberClause(&mut self, ctx: &ReportGroupLineNumberClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberNextPage}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupLineNumberNextPage(&mut self, ctx: &ReportGroupLineNumberNextPageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberPlus}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupLineNumberPlus(&mut self, ctx: &ReportGroupLineNumberPlusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupNextGroupClause(&mut self, ctx: &ReportGroupNextGroupClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupPlus}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupNextGroupPlus(&mut self, ctx: &ReportGroupNextGroupPlusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupNextPage}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupNextGroupNextPage(&mut self, ctx: &ReportGroupNextGroupNextPageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupPictureClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupPictureClause(&mut self, ctx: &ReportGroupPictureClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupResetClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupResetClause(&mut self, ctx: &ReportGroupResetClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupSignClause(&mut self, ctx: &ReportGroupSignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSourceClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupSourceClause(&mut self, ctx: &ReportGroupSourceClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSumClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupSumClause(&mut self, ctx: &ReportGroupSumClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeClause(&mut self, ctx: &ReportGroupTypeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportHeading}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeReportHeading(&mut self, ctx: &ReportGroupTypeReportHeadingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageHeading}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypePageHeading(&mut self, ctx: &ReportGroupTypePageHeadingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlHeading}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeControlHeading(&mut self, ctx: &ReportGroupTypeControlHeadingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeDetail}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeDetail(&mut self, ctx: &ReportGroupTypeDetailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlFooting}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeControlFooting(&mut self, ctx: &ReportGroupTypeControlFootingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupUsageClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupUsageClause(&mut self, ctx: &ReportGroupUsageClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageFooting}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypePageFooting(&mut self, ctx: &ReportGroupTypePageFootingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportFooting}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupTypeReportFooting(&mut self, ctx: &ReportGroupTypeReportFootingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupValueClause}.
	 * @param ctx the parse tree
	 */
	fn visit_reportGroupValueClause(&mut self, ctx: &ReportGroupValueClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programLibrarySection}.
	 * @param ctx the parse tree
	 */
	fn visit_programLibrarySection(&mut self, ctx: &ProgramLibrarySectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryDescriptionEntry(&mut self, ctx: &LibraryDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryDescriptionEntryFormat1(&mut self, ctx: &LibraryDescriptionEntryFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryDescriptionEntryFormat2(&mut self, ctx: &LibraryDescriptionEntryFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryAttributeClauseFormat1(&mut self, ctx: &LibraryAttributeClauseFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryAttributeClauseFormat2(&mut self, ctx: &LibraryAttributeClauseFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryAttributeFunction(&mut self, ctx: &LibraryAttributeFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryAttributeParameter(&mut self, ctx: &LibraryAttributeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeTitle}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryAttributeTitle(&mut self, ctx: &LibraryAttributeTitleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureClauseFormat1(&mut self, ctx: &LibraryEntryProcedureClauseFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureClauseFormat2(&mut self, ctx: &LibraryEntryProcedureClauseFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureForClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureForClause(&mut self, ctx: &LibraryEntryProcedureForClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureGivingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureGivingClause(&mut self, ctx: &LibraryEntryProcedureGivingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureUsingClause(&mut self, ctx: &LibraryEntryProcedureUsingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingName}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureUsingName(&mut self, ctx: &LibraryEntryProcedureUsingNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureWithClause(&mut self, ctx: &LibraryEntryProcedureWithClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithName}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryEntryProcedureWithName(&mut self, ctx: &LibraryEntryProcedureWithNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryIsCommonClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryIsCommonClause(&mut self, ctx: &LibraryIsCommonClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryIsGlobalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryIsGlobalClause(&mut self, ctx: &LibraryIsGlobalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescriptionEntry(&mut self, ctx: &DataDescriptionEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescriptionEntryFormat1(&mut self, ctx: &DataDescriptionEntryFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescriptionEntryFormat2(&mut self, ctx: &DataDescriptionEntryFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescriptionEntryFormat3(&mut self, ctx: &DataDescriptionEntryFormat3Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryExecSql}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescriptionEntryExecSql(&mut self, ctx: &DataDescriptionEntryExecSqlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataAlignedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataAlignedClause(&mut self, ctx: &DataAlignedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataBlankWhenZeroClause(&mut self, ctx: &DataBlankWhenZeroClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataCommonOwnLocalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataCommonOwnLocalClause(&mut self, ctx: &DataCommonOwnLocalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataExternalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataExternalClause(&mut self, ctx: &DataExternalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataGlobalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataGlobalClause(&mut self, ctx: &DataGlobalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataIntegerStringClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataIntegerStringClause(&mut self, ctx: &DataIntegerStringClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataJustifiedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataJustifiedClause(&mut self, ctx: &DataJustifiedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataOccursClause(&mut self, ctx: &DataOccursClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursTo}.
	 * @param ctx the parse tree
	 */
	fn visit_dataOccursTo(&mut self, ctx: &DataOccursToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursSort}.
	 * @param ctx the parse tree
	 */
	fn visit_dataOccursSort(&mut self, ctx: &DataOccursSortContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataPictureClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataPictureClause(&mut self, ctx: &DataPictureClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureString}.
	 * @param ctx the parse tree
	 */
	fn visit_pictureString(&mut self, ctx: &PictureStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureChars}.
	 * @param ctx the parse tree
	 */
	fn visit_pictureChars(&mut self, ctx: &PictureCharsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureCardinality}.
	 * @param ctx the parse tree
	 */
	fn visit_pictureCardinality(&mut self, ctx: &PictureCardinalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataReceivedByClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataReceivedByClause(&mut self, ctx: &DataReceivedByClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRecordAreaClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataRecordAreaClause(&mut self, ctx: &DataRecordAreaClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRedefinesClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataRedefinesClause(&mut self, ctx: &DataRedefinesClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRenamesClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataRenamesClause(&mut self, ctx: &DataRenamesClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataSignClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataSignClause(&mut self, ctx: &DataSignClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataSynchronizedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataSynchronizedClause(&mut self, ctx: &DataSynchronizedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataThreadLocalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataThreadLocalClause(&mut self, ctx: &DataThreadLocalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataTypeClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataTypeClause(&mut self, ctx: &DataTypeClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataTypeDefClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataTypeDefClause(&mut self, ctx: &DataTypeDefClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataUsageClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataUsageClause(&mut self, ctx: &DataUsageClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataUsingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataUsingClause(&mut self, ctx: &DataUsingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataValueClause(&mut self, ctx: &DataValueClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_dataValueInterval(&mut self, ctx: &DataValueIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueIntervalFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_dataValueIntervalFrom(&mut self, ctx: &DataValueIntervalFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueIntervalTo}.
	 * @param ctx the parse tree
	 */
	fn visit_dataValueIntervalTo(&mut self, ctx: &DataValueIntervalToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataWithLowerBoundsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_dataWithLowerBoundsClause(&mut self, ctx: &DataWithLowerBoundsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivision}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivision(&mut self, ctx: &ProcedureDivisionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionUsingClause(&mut self, ctx: &ProcedureDivisionUsingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionGivingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionGivingClause(&mut self, ctx: &ProcedureDivisionGivingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionUsingParameter(&mut self, ctx: &ProcedureDivisionUsingParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReferencePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionByReferencePhrase(&mut self, ctx: &ProcedureDivisionByReferencePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReference}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionByReference(&mut self, ctx: &ProcedureDivisionByReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValuePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionByValuePhrase(&mut self, ctx: &ProcedureDivisionByValuePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValue}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionByValue(&mut self, ctx: &ProcedureDivisionByValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDeclaratives}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDeclaratives(&mut self, ctx: &ProcedureDeclarativesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDeclarative}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDeclarative(&mut self, ctx: &ProcedureDeclarativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureSectionHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureSectionHeader(&mut self, ctx: &ProcedureSectionHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureDivisionBody(&mut self, ctx: &ProcedureDivisionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureSection}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureSection(&mut self, ctx: &ProcedureSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraphs}.
	 * @param ctx the parse tree
	 */
	fn visit_paragraphs(&mut self, ctx: &ParagraphsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraph}.
	 * @param ctx the parse tree
	 */
	fn visit_paragraph(&mut self, ctx: &ParagraphContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sentence}.
	 * @param ctx the parse tree
	 */
	fn visit_sentence(&mut self, ctx: &SentenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_acceptStatement(&mut self, ctx: &AcceptStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromDateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_acceptFromDateStatement(&mut self, ctx: &AcceptFromDateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromMnemonicStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_acceptFromMnemonicStatement(&mut self, ctx: &AcceptFromMnemonicStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromEscapeKeyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_acceptFromEscapeKeyStatement(&mut self, ctx: &AcceptFromEscapeKeyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptMessageCountStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_acceptMessageCountStatement(&mut self, ctx: &AcceptMessageCountStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addStatement(&mut self, ctx: &AddStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addToStatement(&mut self, ctx: &AddToStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToGivingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addToGivingStatement(&mut self, ctx: &AddToGivingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addCorrespondingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addCorrespondingStatement(&mut self, ctx: &AddCorrespondingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_addFrom(&mut self, ctx: &AddFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addTo}.
	 * @param ctx the parse tree
	 */
	fn visit_addTo(&mut self, ctx: &AddToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_addToGiving(&mut self, ctx: &AddToGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_addGiving(&mut self, ctx: &AddGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alteredGoTo}.
	 * @param ctx the parse tree
	 */
	fn visit_alteredGoTo(&mut self, ctx: &AlteredGoToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alterStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterStatement(&mut self, ctx: &AlterStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alterProceedTo}.
	 * @param ctx the parse tree
	 */
	fn visit_alterProceedTo(&mut self, ctx: &AlterProceedToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callUsingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_callUsingPhrase(&mut self, ctx: &CallUsingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callUsingParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_callUsingParameter(&mut self, ctx: &CallUsingParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByReferencePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_callByReferencePhrase(&mut self, ctx: &CallByReferencePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByReference}.
	 * @param ctx the parse tree
	 */
	fn visit_callByReference(&mut self, ctx: &CallByReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByValuePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_callByValuePhrase(&mut self, ctx: &CallByValuePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByValue}.
	 * @param ctx the parse tree
	 */
	fn visit_callByValue(&mut self, ctx: &CallByValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByContentPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_callByContentPhrase(&mut self, ctx: &CallByContentPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByContent}.
	 * @param ctx the parse tree
	 */
	fn visit_callByContent(&mut self, ctx: &CallByContentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callGivingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_callGivingPhrase(&mut self, ctx: &CallGivingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cancelStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_cancelStatement(&mut self, ctx: &CancelStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cancelCall}.
	 * @param ctx the parse tree
	 */
	fn visit_cancelCall(&mut self, ctx: &CancelCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_closeStatement(&mut self, ctx: &CloseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeFile}.
	 * @param ctx the parse tree
	 */
	fn visit_closeFile(&mut self, ctx: &CloseFileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeReelUnitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_closeReelUnitStatement(&mut self, ctx: &CloseReelUnitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeRelativeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_closeRelativeStatement(&mut self, ctx: &CloseRelativeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_closePortFileIOStatement(&mut self, ctx: &ClosePortFileIOStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsing}.
	 * @param ctx the parse tree
	 */
	fn visit_closePortFileIOUsing(&mut self, ctx: &ClosePortFileIOUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingCloseDisposition}.
	 * @param ctx the parse tree
	 */
	fn visit_closePortFileIOUsingCloseDisposition(&mut self, ctx: &ClosePortFileIOUsingCloseDispositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedData}.
	 * @param ctx the parse tree
	 */
	fn visit_closePortFileIOUsingAssociatedData(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedDataLength}.
	 * @param ctx the parse tree
	 */
	fn visit_closePortFileIOUsingAssociatedDataLength(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataLengthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_computeStatement(&mut self, ctx: &ComputeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computeStore}.
	 * @param ctx the parse tree
	 */
	fn visit_computeStore(&mut self, ctx: &ComputeStoreContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#continueStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#deleteStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#disableStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_disableStatement(&mut self, ctx: &DisableStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_displayStatement(&mut self, ctx: &DisplayStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_displayOperand(&mut self, ctx: &DisplayOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayAt}.
	 * @param ctx the parse tree
	 */
	fn visit_displayAt(&mut self, ctx: &DisplayAtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayUpon}.
	 * @param ctx the parse tree
	 */
	fn visit_displayUpon(&mut self, ctx: &DisplayUponContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayWith}.
	 * @param ctx the parse tree
	 */
	fn visit_displayWith(&mut self, ctx: &DisplayWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_divideStatement(&mut self, ctx: &DivideStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideIntoStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_divideIntoStatement(&mut self, ctx: &DivideIntoStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideIntoGivingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_divideIntoGivingStatement(&mut self, ctx: &DivideIntoGivingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideByGivingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_divideByGivingStatement(&mut self, ctx: &DivideByGivingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideGivingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_divideGivingPhrase(&mut self, ctx: &DivideGivingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideInto}.
	 * @param ctx the parse tree
	 */
	fn visit_divideInto(&mut self, ctx: &DivideIntoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_divideGiving(&mut self, ctx: &DivideGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideRemainder}.
	 * @param ctx the parse tree
	 */
	fn visit_divideRemainder(&mut self, ctx: &DivideRemainderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#enableStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_enableStatement(&mut self, ctx: &EnableStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#entryStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_entryStatement(&mut self, ctx: &EntryStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateSelect}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateSelect(&mut self, ctx: &EvaluateSelectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateAlsoSelect}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateAlsoSelect(&mut self, ctx: &EvaluateAlsoSelectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhenPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateWhenPhrase(&mut self, ctx: &EvaluateWhenPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhen}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateWhen(&mut self, ctx: &EvaluateWhenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateCondition(&mut self, ctx: &EvaluateConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateThrough(&mut self, ctx: &EvaluateThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateAlsoCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateAlsoCondition(&mut self, ctx: &EvaluateAlsoConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhenOther}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateWhenOther(&mut self, ctx: &EvaluateWhenOtherContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateValue}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateValue(&mut self, ctx: &EvaluateValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execCicsStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execSqlStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execSqlImsStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exhibitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_exhibitStatement(&mut self, ctx: &ExhibitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exhibitOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_exhibitOperand(&mut self, ctx: &ExhibitOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_exitStatement(&mut self, ctx: &ExitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#generateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_generateStatement(&mut self, ctx: &GenerateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#gobackStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_gobackStatement(&mut self, ctx: &GobackStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_goToStatement(&mut self, ctx: &GoToStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToStatementSimple}.
	 * @param ctx the parse tree
	 */
	fn visit_goToStatementSimple(&mut self, ctx: &GoToStatementSimpleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToDependingOnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_goToDependingOnStatement(&mut self, ctx: &GoToDependingOnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifThen}.
	 * @param ctx the parse tree
	 */
	fn visit_ifThen(&mut self, ctx: &IfThenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifElse}.
	 * @param ctx the parse tree
	 */
	fn visit_ifElse(&mut self, ctx: &IfElseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_initializeStatement(&mut self, ctx: &InitializeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeReplacingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_initializeReplacingPhrase(&mut self, ctx: &InitializeReplacingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeReplacingBy}.
	 * @param ctx the parse tree
	 */
	fn visit_initializeReplacingBy(&mut self, ctx: &InitializeReplacingByContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initiateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_initiateStatement(&mut self, ctx: &InitiateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectStatement(&mut self, ctx: &InspectStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTallyingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectTallyingPhrase(&mut self, ctx: &InspectTallyingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectReplacingPhrase(&mut self, ctx: &InspectReplacingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTallyingReplacingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectTallyingReplacingPhrase(&mut self, ctx: &InspectTallyingReplacingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectConvertingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectConvertingPhrase(&mut self, ctx: &InspectConvertingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectFor}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectFor(&mut self, ctx: &InspectForContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectCharacters}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectCharacters(&mut self, ctx: &InspectCharactersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingCharacters}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectReplacingCharacters(&mut self, ctx: &InspectReplacingCharactersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectAllLeadings}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectAllLeadings(&mut self, ctx: &InspectAllLeadingsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeadings}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectReplacingAllLeadings(&mut self, ctx: &InspectReplacingAllLeadingsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectAllLeading}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectAllLeading(&mut self, ctx: &InspectAllLeadingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeading}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectReplacingAllLeading(&mut self, ctx: &InspectReplacingAllLeadingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectBy}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectBy(&mut self, ctx: &InspectByContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTo}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectTo(&mut self, ctx: &InspectToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectBeforeAfter}.
	 * @param ctx the parse tree
	 */
	fn visit_inspectBeforeAfter(&mut self, ctx: &InspectBeforeAfterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeStatement(&mut self, ctx: &MergeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOnKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeOnKeyClause(&mut self, ctx: &MergeOnKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingSequencePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeCollatingSequencePhrase(&mut self, ctx: &MergeCollatingSequencePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingAlphanumeric}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeCollatingAlphanumeric(&mut self, ctx: &MergeCollatingAlphanumericContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingNational}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeCollatingNational(&mut self, ctx: &MergeCollatingNationalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeUsing}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeUsing(&mut self, ctx: &MergeUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOutputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeOutputProcedurePhrase(&mut self, ctx: &MergeOutputProcedurePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOutputThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeOutputThrough(&mut self, ctx: &MergeOutputThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeGivingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeGivingPhrase(&mut self, ctx: &MergeGivingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeGiving(&mut self, ctx: &MergeGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_moveStatement(&mut self, ctx: &MoveStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveToStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_moveToStatement(&mut self, ctx: &MoveToStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveToSendingArea}.
	 * @param ctx the parse tree
	 */
	fn visit_moveToSendingArea(&mut self, ctx: &MoveToSendingAreaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_moveCorrespondingToStatement(&mut self, ctx: &MoveCorrespondingToStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToSendingArea}.
	 * @param ctx the parse tree
	 */
	fn visit_moveCorrespondingToSendingArea(&mut self, ctx: &MoveCorrespondingToSendingAreaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyStatement(&mut self, ctx: &MultiplyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyRegular}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyRegular(&mut self, ctx: &MultiplyRegularContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyRegularOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyRegularOperand(&mut self, ctx: &MultiplyRegularOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyGiving(&mut self, ctx: &MultiplyGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGivingOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyGivingOperand(&mut self, ctx: &MultiplyGivingOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGivingResult}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyGivingResult(&mut self, ctx: &MultiplyGivingResultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_openStatement(&mut self, ctx: &OpenStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openInputStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_openInputStatement(&mut self, ctx: &OpenInputStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openInput}.
	 * @param ctx the parse tree
	 */
	fn visit_openInput(&mut self, ctx: &OpenInputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openOutputStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_openOutputStatement(&mut self, ctx: &OpenOutputStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openOutput}.
	 * @param ctx the parse tree
	 */
	fn visit_openOutput(&mut self, ctx: &OpenOutputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openIOStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_openIOStatement(&mut self, ctx: &OpenIOStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openExtendStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_openExtendStatement(&mut self, ctx: &OpenExtendStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_performStatement(&mut self, ctx: &PerformStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performInlineStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_performInlineStatement(&mut self, ctx: &PerformInlineStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performProcedureStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_performProcedureStatement(&mut self, ctx: &PerformProcedureStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performType}.
	 * @param ctx the parse tree
	 */
	fn visit_performType(&mut self, ctx: &PerformTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performTimes}.
	 * @param ctx the parse tree
	 */
	fn visit_performTimes(&mut self, ctx: &PerformTimesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performUntil}.
	 * @param ctx the parse tree
	 */
	fn visit_performUntil(&mut self, ctx: &PerformUntilContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVarying}.
	 * @param ctx the parse tree
	 */
	fn visit_performVarying(&mut self, ctx: &PerformVaryingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVaryingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_performVaryingClause(&mut self, ctx: &PerformVaryingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVaryingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_performVaryingPhrase(&mut self, ctx: &PerformVaryingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performAfter}.
	 * @param ctx the parse tree
	 */
	fn visit_performAfter(&mut self, ctx: &PerformAfterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_performFrom(&mut self, ctx: &PerformFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performBy}.
	 * @param ctx the parse tree
	 */
	fn visit_performBy(&mut self, ctx: &PerformByContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performTestClause}.
	 * @param ctx the parse tree
	 */
	fn visit_performTestClause(&mut self, ctx: &PerformTestClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#purgeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_purgeStatement(&mut self, ctx: &PurgeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_readStatement(&mut self, ctx: &ReadStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readInto}.
	 * @param ctx the parse tree
	 */
	fn visit_readInto(&mut self, ctx: &ReadIntoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readWith}.
	 * @param ctx the parse tree
	 */
	fn visit_readWith(&mut self, ctx: &ReadWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readKey}.
	 * @param ctx the parse tree
	 */
	fn visit_readKey(&mut self, ctx: &ReadKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveStatement(&mut self, ctx: &ReceiveStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveFromStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveFromStatement(&mut self, ctx: &ReceiveFromStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveFrom(&mut self, ctx: &ReceiveFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveIntoStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveIntoStatement(&mut self, ctx: &ReceiveIntoStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveNoData}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveNoData(&mut self, ctx: &ReceiveNoDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveWithData}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveWithData(&mut self, ctx: &ReceiveWithDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveBefore}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveBefore(&mut self, ctx: &ReceiveBeforeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveWith}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveWith(&mut self, ctx: &ReceiveWithContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveThread}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveThread(&mut self, ctx: &ReceiveThreadContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveSize}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveSize(&mut self, ctx: &ReceiveSizeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveStatus}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveStatus(&mut self, ctx: &ReceiveStatusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#releaseStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_releaseStatement(&mut self, ctx: &ReleaseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#returnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#returnInto}.
	 * @param ctx the parse tree
	 */
	fn visit_returnInto(&mut self, ctx: &ReturnIntoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rewriteStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_rewriteStatement(&mut self, ctx: &RewriteStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rewriteFrom}.
	 * @param ctx the parse tree
	 */
	fn visit_rewriteFrom(&mut self, ctx: &RewriteFromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_searchStatement(&mut self, ctx: &SearchStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchVarying}.
	 * @param ctx the parse tree
	 */
	fn visit_searchVarying(&mut self, ctx: &SearchVaryingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchWhen}.
	 * @param ctx the parse tree
	 */
	fn visit_searchWhen(&mut self, ctx: &SearchWhenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_sendStatement(&mut self, ctx: &SendStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatementSync}.
	 * @param ctx the parse tree
	 */
	fn visit_sendStatementSync(&mut self, ctx: &SendStatementSyncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatementAsync}.
	 * @param ctx the parse tree
	 */
	fn visit_sendStatementAsync(&mut self, ctx: &SendStatementAsyncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendFromPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sendFromPhrase(&mut self, ctx: &SendFromPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendWithPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sendWithPhrase(&mut self, ctx: &SendWithPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendReplacingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sendReplacingPhrase(&mut self, ctx: &SendReplacingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sendAdvancingPhrase(&mut self, ctx: &SendAdvancingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingPage}.
	 * @param ctx the parse tree
	 */
	fn visit_sendAdvancingPage(&mut self, ctx: &SendAdvancingPageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingLines}.
	 * @param ctx the parse tree
	 */
	fn visit_sendAdvancingLines(&mut self, ctx: &SendAdvancingLinesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingMnemonic}.
	 * @param ctx the parse tree
	 */
	fn visit_sendAdvancingMnemonic(&mut self, ctx: &SendAdvancingMnemonicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setToStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setToStatement(&mut self, ctx: &SetToStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setUpDownByStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setUpDownByStatement(&mut self, ctx: &SetUpDownByStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setTo}.
	 * @param ctx the parse tree
	 */
	fn visit_setTo(&mut self, ctx: &SetToContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setToValue}.
	 * @param ctx the parse tree
	 */
	fn visit_setToValue(&mut self, ctx: &SetToValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setByValue}.
	 * @param ctx the parse tree
	 */
	fn visit_setByValue(&mut self, ctx: &SetByValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_sortStatement(&mut self, ctx: &SortStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOnKeyClause}.
	 * @param ctx the parse tree
	 */
	fn visit_sortOnKeyClause(&mut self, ctx: &SortOnKeyClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortDuplicatesPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sortDuplicatesPhrase(&mut self, ctx: &SortDuplicatesPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingSequencePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sortCollatingSequencePhrase(&mut self, ctx: &SortCollatingSequencePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingAlphanumeric}.
	 * @param ctx the parse tree
	 */
	fn visit_sortCollatingAlphanumeric(&mut self, ctx: &SortCollatingAlphanumericContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingNational}.
	 * @param ctx the parse tree
	 */
	fn visit_sortCollatingNational(&mut self, ctx: &SortCollatingNationalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortInputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sortInputProcedurePhrase(&mut self, ctx: &SortInputProcedurePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortInputThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_sortInputThrough(&mut self, ctx: &SortInputThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortUsing}.
	 * @param ctx the parse tree
	 */
	fn visit_sortUsing(&mut self, ctx: &SortUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOutputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sortOutputProcedurePhrase(&mut self, ctx: &SortOutputProcedurePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOutputThrough}.
	 * @param ctx the parse tree
	 */
	fn visit_sortOutputThrough(&mut self, ctx: &SortOutputThroughContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortGivingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_sortGivingPhrase(&mut self, ctx: &SortGivingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_sortGiving(&mut self, ctx: &SortGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_startStatement(&mut self, ctx: &StartStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startKey}.
	 * @param ctx the parse tree
	 */
	fn visit_startKey(&mut self, ctx: &StartKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stopStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_stopStatement(&mut self, ctx: &StopStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringStatement(&mut self, ctx: &StringStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringSendingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_stringSendingPhrase(&mut self, ctx: &StringSendingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringSending}.
	 * @param ctx the parse tree
	 */
	fn visit_stringSending(&mut self, ctx: &StringSendingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringDelimitedByPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_stringDelimitedByPhrase(&mut self, ctx: &StringDelimitedByPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringForPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_stringForPhrase(&mut self, ctx: &StringForPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringIntoPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_stringIntoPhrase(&mut self, ctx: &StringIntoPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringWithPointerPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_stringWithPointerPhrase(&mut self, ctx: &StringWithPointerPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractStatement(&mut self, ctx: &SubtractStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractFromStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractFromStatement(&mut self, ctx: &SubtractFromStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractFromGivingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractFromGivingStatement(&mut self, ctx: &SubtractFromGivingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractCorrespondingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractCorrespondingStatement(&mut self, ctx: &SubtractCorrespondingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractSubtrahend}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractSubtrahend(&mut self, ctx: &SubtractSubtrahendContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuend}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractMinuend(&mut self, ctx: &SubtractMinuendContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuendGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractMinuendGiving(&mut self, ctx: &SubtractMinuendGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractGiving}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractGiving(&mut self, ctx: &SubtractGivingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuendCorresponding}.
	 * @param ctx the parse tree
	 */
	fn visit_subtractMinuendCorresponding(&mut self, ctx: &SubtractMinuendCorrespondingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#terminateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_terminateStatement(&mut self, ctx: &TerminateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringStatement(&mut self, ctx: &UnstringStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringSendingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringSendingPhrase(&mut self, ctx: &UnstringSendingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringDelimitedByPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringDelimitedByPhrase(&mut self, ctx: &UnstringDelimitedByPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringOrAllPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringOrAllPhrase(&mut self, ctx: &UnstringOrAllPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringIntoPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringIntoPhrase(&mut self, ctx: &UnstringIntoPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringInto}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringInto(&mut self, ctx: &UnstringIntoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringDelimiterIn}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringDelimiterIn(&mut self, ctx: &UnstringDelimiterInContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringCountIn}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringCountIn(&mut self, ctx: &UnstringCountInContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringWithPointerPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringWithPointerPhrase(&mut self, ctx: &UnstringWithPointerPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringTallyingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_unstringTallyingPhrase(&mut self, ctx: &UnstringTallyingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_useStatement(&mut self, ctx: &UseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useAfterClause}.
	 * @param ctx the parse tree
	 */
	fn visit_useAfterClause(&mut self, ctx: &UseAfterClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useAfterOn}.
	 * @param ctx the parse tree
	 */
	fn visit_useAfterOn(&mut self, ctx: &UseAfterOnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useDebugClause}.
	 * @param ctx the parse tree
	 */
	fn visit_useDebugClause(&mut self, ctx: &UseDebugClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useDebugOn}.
	 * @param ctx the parse tree
	 */
	fn visit_useDebugOn(&mut self, ctx: &UseDebugOnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_writeStatement(&mut self, ctx: &WriteStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeFromPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_writeFromPhrase(&mut self, ctx: &WriteFromPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_writeAdvancingPhrase(&mut self, ctx: &WriteAdvancingPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingPage}.
	 * @param ctx the parse tree
	 */
	fn visit_writeAdvancingPage(&mut self, ctx: &WriteAdvancingPageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingLines}.
	 * @param ctx the parse tree
	 */
	fn visit_writeAdvancingLines(&mut self, ctx: &WriteAdvancingLinesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingMnemonic}.
	 * @param ctx the parse tree
	 */
	fn visit_writeAdvancingMnemonic(&mut self, ctx: &WriteAdvancingMnemonicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAtEndOfPagePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_writeAtEndOfPagePhrase(&mut self, ctx: &WriteAtEndOfPagePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeNotAtEndOfPagePhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_writeNotAtEndOfPagePhrase(&mut self, ctx: &WriteNotAtEndOfPagePhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#atEndPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_atEndPhrase(&mut self, ctx: &AtEndPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notAtEndPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_notAtEndPhrase(&mut self, ctx: &NotAtEndPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#invalidKeyPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_invalidKeyPhrase(&mut self, ctx: &InvalidKeyPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notInvalidKeyPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_notInvalidKeyPhrase(&mut self, ctx: &NotInvalidKeyPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onOverflowPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_onOverflowPhrase(&mut self, ctx: &OnOverflowPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnOverflowPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_notOnOverflowPhrase(&mut self, ctx: &NotOnOverflowPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onSizeErrorPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_onSizeErrorPhrase(&mut self, ctx: &OnSizeErrorPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnSizeErrorPhrase}.
	 * @param ctx the parse tree
	 */
	fn visit_notOnSizeErrorPhrase(&mut self, ctx: &NotOnSizeErrorPhraseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onExceptionClause}.
	 * @param ctx the parse tree
	 */
	fn visit_onExceptionClause(&mut self, ctx: &OnExceptionClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnExceptionClause}.
	 * @param ctx the parse tree
	 */
	fn visit_notOnExceptionClause(&mut self, ctx: &NotOnExceptionClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#arithmeticExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticExpression(&mut self, ctx: &ArithmeticExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#plusMinus}.
	 * @param ctx the parse tree
	 */
	fn visit_plusMinus(&mut self, ctx: &PlusMinusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multDivs}.
	 * @param ctx the parse tree
	 */
	fn visit_multDivs(&mut self, ctx: &MultDivsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multDiv}.
	 * @param ctx the parse tree
	 */
	fn visit_multDiv(&mut self, ctx: &MultDivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#powers}.
	 * @param ctx the parse tree
	 */
	fn visit_powers(&mut self, ctx: &PowersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#power}.
	 * @param ctx the parse tree
	 */
	fn visit_power(&mut self, ctx: &PowerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#basis}.
	 * @param ctx the parse tree
	 */
	fn visit_basis(&mut self, ctx: &BasisContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#condition}.
	 * @param ctx the parse tree
	 */
	fn visit_condition(&mut self, ctx: &ConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#andOrCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_andOrCondition(&mut self, ctx: &AndOrConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#combinableCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_combinableCondition(&mut self, ctx: &CombinableConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#simpleCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleCondition(&mut self, ctx: &SimpleConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_classCondition(&mut self, ctx: &ClassConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionNameReference}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionNameReference(&mut self, ctx: &ConditionNameReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionNameSubscriptReference}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionNameSubscriptReference(&mut self, ctx: &ConditionNameSubscriptReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_relationCondition(&mut self, ctx: &RelationConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationSignCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_relationSignCondition(&mut self, ctx: &RelationSignConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationArithmeticComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_relationArithmeticComparison(&mut self, ctx: &RelationArithmeticComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCombinedComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_relationCombinedComparison(&mut self, ctx: &RelationCombinedComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCombinedCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_relationCombinedCondition(&mut self, ctx: &RelationCombinedConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationalOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#abbreviation}.
	 * @param ctx the parse tree
	 */
	fn visit_abbreviation(&mut self, ctx: &AbbreviationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#tableCall}.
	 * @param ctx the parse tree
	 */
	fn visit_tableCall(&mut self, ctx: &TableCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#referenceModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_referenceModifier(&mut self, ctx: &ReferenceModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#characterPosition}.
	 * @param ctx the parse tree
	 */
	fn visit_characterPosition(&mut self, ctx: &CharacterPositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#length}.
	 * @param ctx the parse tree
	 */
	fn visit_length(&mut self, ctx: &LengthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subscript_}.
	 * @param ctx the parse tree
	 */
	fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#argument}.
	 * @param ctx the parse tree
	 */
	fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataName}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedDataName(&mut self, ctx: &QualifiedDataNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat1}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedDataNameFormat1(&mut self, ctx: &QualifiedDataNameFormat1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat2}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedDataNameFormat2(&mut self, ctx: &QualifiedDataNameFormat2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat3}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedDataNameFormat3(&mut self, ctx: &QualifiedDataNameFormat3Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat4}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedDataNameFormat4(&mut self, ctx: &QualifiedDataNameFormat4Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedInData}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedInData(&mut self, ctx: &QualifiedInDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inData}.
	 * @param ctx the parse tree
	 */
	fn visit_inData(&mut self, ctx: &InDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inFile}.
	 * @param ctx the parse tree
	 */
	fn visit_inFile(&mut self, ctx: &InFileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inMnemonic}.
	 * @param ctx the parse tree
	 */
	fn visit_inMnemonic(&mut self, ctx: &InMnemonicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inSection}.
	 * @param ctx the parse tree
	 */
	fn visit_inSection(&mut self, ctx: &InSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inLibrary}.
	 * @param ctx the parse tree
	 */
	fn visit_inLibrary(&mut self, ctx: &InLibraryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inTable}.
	 * @param ctx the parse tree
	 */
	fn visit_inTable(&mut self, ctx: &InTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetName}.
	 * @param ctx the parse tree
	 */
	fn visit_alphabetName(&mut self, ctx: &AlphabetNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#assignmentName}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentName(&mut self, ctx: &AssignmentNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#basisName}.
	 * @param ctx the parse tree
	 */
	fn visit_basisName(&mut self, ctx: &BasisNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cdName}.
	 * @param ctx the parse tree
	 */
	fn visit_cdName(&mut self, ctx: &CdNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#className}.
	 * @param ctx the parse tree
	 */
	fn visit_className(&mut self, ctx: &ClassNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computerName}.
	 * @param ctx the parse tree
	 */
	fn visit_computerName(&mut self, ctx: &ComputerNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionName}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionName(&mut self, ctx: &ConditionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataName}.
	 * @param ctx the parse tree
	 */
	fn visit_dataName(&mut self, ctx: &DataNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescName}.
	 * @param ctx the parse tree
	 */
	fn visit_dataDescName(&mut self, ctx: &DataDescNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentName}.
	 * @param ctx the parse tree
	 */
	fn visit_environmentName(&mut self, ctx: &EnvironmentNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileName}.
	 * @param ctx the parse tree
	 */
	fn visit_fileName(&mut self, ctx: &FileNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#functionName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#indexName}.
	 * @param ctx the parse tree
	 */
	fn visit_indexName(&mut self, ctx: &IndexNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#languageName}.
	 * @param ctx the parse tree
	 */
	fn visit_languageName(&mut self, ctx: &LanguageNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryName}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryName(&mut self, ctx: &LibraryNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#localName}.
	 * @param ctx the parse tree
	 */
	fn visit_localName(&mut self, ctx: &LocalNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mnemonicName}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonicName(&mut self, ctx: &MnemonicNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraphName}.
	 * @param ctx the parse tree
	 */
	fn visit_paragraphName(&mut self, ctx: &ParagraphNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureName}.
	 * @param ctx the parse tree
	 */
	fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programName}.
	 * @param ctx the parse tree
	 */
	fn visit_programName(&mut self, ctx: &ProgramNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordName}.
	 * @param ctx the parse tree
	 */
	fn visit_recordName(&mut self, ctx: &RecordNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportName}.
	 * @param ctx the parse tree
	 */
	fn visit_reportName(&mut self, ctx: &ReportNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#routineName}.
	 * @param ctx the parse tree
	 */
	fn visit_routineName(&mut self, ctx: &RoutineNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenName}.
	 * @param ctx the parse tree
	 */
	fn visit_screenName(&mut self, ctx: &ScreenNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sectionName}.
	 * @param ctx the parse tree
	 */
	fn visit_sectionName(&mut self, ctx: &SectionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#systemName}.
	 * @param ctx the parse tree
	 */
	fn visit_systemName(&mut self, ctx: &SystemNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharacter}.
	 * @param ctx the parse tree
	 */
	fn visit_symbolicCharacter(&mut self, ctx: &SymbolicCharacterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#textName}.
	 * @param ctx the parse tree
	 */
	fn visit_textName(&mut self, ctx: &TextNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cobolWord}.
	 * @param ctx the parse tree
	 */
	fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#numericLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#integerLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cicsDfhRespLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_cicsDfhRespLiteral(&mut self, ctx: &CicsDfhRespLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cicsDfhValueLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_cicsDfhValueLiteral(&mut self, ctx: &CicsDfhValueLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#figurativeConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_figurativeConstant(&mut self, ctx: &FigurativeConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialRegister}.
	 * @param ctx the parse tree
	 */
	fn visit_specialRegister(&mut self, ctx: &SpecialRegisterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#commentEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_commentEntry(&mut self, ctx: &CommentEntryContext<'input>) { self.visit_children(ctx) }

}

pub trait Cobol85VisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= Cobol85ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startRule}.
	 * @param ctx the parse tree
	 */
		fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#compilationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_programUnit(&mut self, ctx: &ProgramUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#endProgramStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_endProgramStatement(&mut self, ctx: &EndProgramStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identificationDivision}.
	 * @param ctx the parse tree
	 */
		fn visit_identificationDivision(&mut self, ctx: &IdentificationDivisionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identificationDivisionBody}.
	 * @param ctx the parse tree
	 */
		fn visit_identificationDivisionBody(&mut self, ctx: &IdentificationDivisionBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programIdParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_programIdParagraph(&mut self, ctx: &ProgramIdParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#authorParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_authorParagraph(&mut self, ctx: &AuthorParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#installationParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_installationParagraph(&mut self, ctx: &InstallationParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dateWrittenParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_dateWrittenParagraph(&mut self, ctx: &DateWrittenParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dateCompiledParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_dateCompiledParagraph(&mut self, ctx: &DateCompiledParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#securityParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_securityParagraph(&mut self, ctx: &SecurityParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#remarksParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_remarksParagraph(&mut self, ctx: &RemarksParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentDivision}.
	 * @param ctx the parse tree
	 */
		fn visit_environmentDivision(&mut self, ctx: &EnvironmentDivisionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentDivisionBody}.
	 * @param ctx the parse tree
	 */
		fn visit_environmentDivisionBody(&mut self, ctx: &EnvironmentDivisionBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#configurationSection}.
	 * @param ctx the parse tree
	 */
		fn visit_configurationSection(&mut self, ctx: &ConfigurationSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#configurationSectionParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_configurationSectionParagraph(&mut self, ctx: &ConfigurationSectionParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sourceComputerParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_sourceComputerParagraph(&mut self, ctx: &SourceComputerParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#objectComputerParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_objectComputerParagraph(&mut self, ctx: &ObjectComputerParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#objectComputerClause}.
	 * @param ctx the parse tree
	 */
		fn visit_objectComputerClause(&mut self, ctx: &ObjectComputerClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#memorySizeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_memorySizeClause(&mut self, ctx: &MemorySizeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#diskSizeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_diskSizeClause(&mut self, ctx: &DiskSizeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClause}.
	 * @param ctx the parse tree
	 */
		fn visit_collatingSequenceClause(&mut self, ctx: &CollatingSequenceClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseAlphanumeric}.
	 * @param ctx the parse tree
	 */
		fn visit_collatingSequenceClauseAlphanumeric(&mut self, ctx: &CollatingSequenceClauseAlphanumericContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseNational}.
	 * @param ctx the parse tree
	 */
		fn visit_collatingSequenceClauseNational(&mut self, ctx: &CollatingSequenceClauseNationalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#segmentLimitClause}.
	 * @param ctx the parse tree
	 */
		fn visit_segmentLimitClause(&mut self, ctx: &SegmentLimitClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#characterSetClause}.
	 * @param ctx the parse tree
	 */
		fn visit_characterSetClause(&mut self, ctx: &CharacterSetClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialNamesParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_specialNamesParagraph(&mut self, ctx: &SpecialNamesParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialNameClause}.
	 * @param ctx the parse tree
	 */
		fn visit_specialNameClause(&mut self, ctx: &SpecialNameClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClause}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetClause(&mut self, ctx: &AlphabetClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetClauseFormat1(&mut self, ctx: &AlphabetClauseFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetLiterals}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetLiterals(&mut self, ctx: &AlphabetLiteralsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetThrough(&mut self, ctx: &AlphabetThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetAlso}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetAlso(&mut self, ctx: &AlphabetAlsoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetClauseFormat2(&mut self, ctx: &AlphabetClauseFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#channelClause}.
	 * @param ctx the parse tree
	 */
		fn visit_channelClause(&mut self, ctx: &ChannelClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClause}.
	 * @param ctx the parse tree
	 */
		fn visit_classClause(&mut self, ctx: &ClassClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_classClauseThrough(&mut self, ctx: &ClassClauseThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_classClauseFrom(&mut self, ctx: &ClassClauseFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classClauseTo}.
	 * @param ctx the parse tree
	 */
		fn visit_classClauseTo(&mut self, ctx: &ClassClauseToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#currencySignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_currencySignClause(&mut self, ctx: &CurrencySignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#decimalPointClause}.
	 * @param ctx the parse tree
	 */
		fn visit_decimalPointClause(&mut self, ctx: &DecimalPointClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#defaultComputationalSignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_defaultComputationalSignClause(&mut self, ctx: &DefaultComputationalSignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#defaultDisplaySignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_defaultDisplaySignClause(&mut self, ctx: &DefaultDisplaySignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameClause}.
	 * @param ctx the parse tree
	 */
		fn visit_environmentSwitchNameClause(&mut self, ctx: &EnvironmentSwitchNameClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameSpecialNamesStatusPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_environmentSwitchNameSpecialNamesStatusPhrase(&mut self, ctx: &EnvironmentSwitchNameSpecialNamesStatusPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#odtClause}.
	 * @param ctx the parse tree
	 */
		fn visit_odtClause(&mut self, ctx: &OdtClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reserveNetworkClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reserveNetworkClause(&mut self, ctx: &ReserveNetworkClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharactersClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicCharactersClause(&mut self, ctx: &SymbolicCharactersClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharacters}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicCharacters(&mut self, ctx: &SymbolicCharactersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inputOutputSection}.
	 * @param ctx the parse tree
	 */
		fn visit_inputOutputSection(&mut self, ctx: &InputOutputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inputOutputSectionParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_inputOutputSectionParagraph(&mut self, ctx: &InputOutputSectionParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_fileControlParagraph(&mut self, ctx: &FileControlParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_fileControlEntry(&mut self, ctx: &FileControlEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#selectClause}.
	 * @param ctx the parse tree
	 */
		fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileControlClause}.
	 * @param ctx the parse tree
	 */
		fn visit_fileControlClause(&mut self, ctx: &FileControlClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#assignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_assignClause(&mut self, ctx: &AssignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reserveClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reserveClause(&mut self, ctx: &ReserveClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#organizationClause}.
	 * @param ctx the parse tree
	 */
		fn visit_organizationClause(&mut self, ctx: &OrganizationClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paddingCharacterClause}.
	 * @param ctx the parse tree
	 */
		fn visit_paddingCharacterClause(&mut self, ctx: &PaddingCharacterClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordDelimiterClause}.
	 * @param ctx the parse tree
	 */
		fn visit_recordDelimiterClause(&mut self, ctx: &RecordDelimiterClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#accessModeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_accessModeClause(&mut self, ctx: &AccessModeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_recordKeyClause(&mut self, ctx: &RecordKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alternateRecordKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_alternateRecordKeyClause(&mut self, ctx: &AlternateRecordKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#passwordClause}.
	 * @param ctx the parse tree
	 */
		fn visit_passwordClause(&mut self, ctx: &PasswordClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileStatusClause}.
	 * @param ctx the parse tree
	 */
		fn visit_fileStatusClause(&mut self, ctx: &FileStatusClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relativeKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_relativeKeyClause(&mut self, ctx: &RelativeKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ioControlParagraph}.
	 * @param ctx the parse tree
	 */
		fn visit_ioControlParagraph(&mut self, ctx: &IoControlParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ioControlClause}.
	 * @param ctx the parse tree
	 */
		fn visit_ioControlClause(&mut self, ctx: &IoControlClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunClause}.
	 * @param ctx the parse tree
	 */
		fn visit_rerunClause(&mut self, ctx: &RerunClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryRecords}.
	 * @param ctx the parse tree
	 */
		fn visit_rerunEveryRecords(&mut self, ctx: &RerunEveryRecordsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryOf}.
	 * @param ctx the parse tree
	 */
		fn visit_rerunEveryOf(&mut self, ctx: &RerunEveryOfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rerunEveryClock}.
	 * @param ctx the parse tree
	 */
		fn visit_rerunEveryClock(&mut self, ctx: &RerunEveryClockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sameClause}.
	 * @param ctx the parse tree
	 */
		fn visit_sameClause(&mut self, ctx: &SameClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multipleFileClause}.
	 * @param ctx the parse tree
	 */
		fn visit_multipleFileClause(&mut self, ctx: &MultipleFileClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multipleFilePosition}.
	 * @param ctx the parse tree
	 */
		fn visit_multipleFilePosition(&mut self, ctx: &MultipleFilePositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#commitmentControlClause}.
	 * @param ctx the parse tree
	 */
		fn visit_commitmentControlClause(&mut self, ctx: &CommitmentControlClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDivision}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDivision(&mut self, ctx: &DataDivisionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDivisionSection}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDivisionSection(&mut self, ctx: &DataDivisionSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileSection}.
	 * @param ctx the parse tree
	 */
		fn visit_fileSection(&mut self, ctx: &FileSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_fileDescriptionEntry(&mut self, ctx: &FileDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntryClause}.
	 * @param ctx the parse tree
	 */
		fn visit_fileDescriptionEntryClause(&mut self, ctx: &FileDescriptionEntryClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#externalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_externalClause(&mut self, ctx: &ExternalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#globalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_globalClause(&mut self, ctx: &GlobalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#blockContainsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_blockContainsClause(&mut self, ctx: &BlockContainsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#blockContainsTo}.
	 * @param ctx the parse tree
	 */
		fn visit_blockContainsTo(&mut self, ctx: &BlockContainsToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_recordContainsClause(&mut self, ctx: &RecordContainsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_recordContainsClauseFormat1(&mut self, ctx: &RecordContainsClauseFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_recordContainsClauseFormat2(&mut self, ctx: &RecordContainsClauseFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat3}.
	 * @param ctx the parse tree
	 */
		fn visit_recordContainsClauseFormat3(&mut self, ctx: &RecordContainsClauseFormat3Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordContainsTo}.
	 * @param ctx the parse tree
	 */
		fn visit_recordContainsTo(&mut self, ctx: &RecordContainsToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#labelRecordsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_labelRecordsClause(&mut self, ctx: &LabelRecordsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#valueOfClause}.
	 * @param ctx the parse tree
	 */
		fn visit_valueOfClause(&mut self, ctx: &ValueOfClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#valuePair}.
	 * @param ctx the parse tree
	 */
		fn visit_valuePair(&mut self, ctx: &ValuePairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRecordsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataRecordsClause(&mut self, ctx: &DataRecordsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageClause}.
	 * @param ctx the parse tree
	 */
		fn visit_linageClause(&mut self, ctx: &LinageClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageAt}.
	 * @param ctx the parse tree
	 */
		fn visit_linageAt(&mut self, ctx: &LinageAtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageFootingAt}.
	 * @param ctx the parse tree
	 */
		fn visit_linageFootingAt(&mut self, ctx: &LinageFootingAtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageLinesAtTop}.
	 * @param ctx the parse tree
	 */
		fn visit_linageLinesAtTop(&mut self, ctx: &LinageLinesAtTopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linageLinesAtBottom}.
	 * @param ctx the parse tree
	 */
		fn visit_linageLinesAtBottom(&mut self, ctx: &LinageLinesAtBottomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordingModeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_recordingModeClause(&mut self, ctx: &RecordingModeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#modeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_modeStatement(&mut self, ctx: &ModeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#codeSetClause}.
	 * @param ctx the parse tree
	 */
		fn visit_codeSetClause(&mut self, ctx: &CodeSetClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportClause(&mut self, ctx: &ReportClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBaseSection}.
	 * @param ctx the parse tree
	 */
		fn visit_dataBaseSection(&mut self, ctx: &DataBaseSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBaseSectionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_dataBaseSectionEntry(&mut self, ctx: &DataBaseSectionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#workingStorageSection}.
	 * @param ctx the parse tree
	 */
		fn visit_workingStorageSection(&mut self, ctx: &WorkingStorageSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#linkageSection}.
	 * @param ctx the parse tree
	 */
		fn visit_linkageSection(&mut self, ctx: &LinkageSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationSection}.
	 * @param ctx the parse tree
	 */
		fn visit_communicationSection(&mut self, ctx: &CommunicationSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_communicationDescriptionEntry(&mut self, ctx: &CommunicationDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_communicationDescriptionEntryFormat1(&mut self, ctx: &CommunicationDescriptionEntryFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_communicationDescriptionEntryFormat2(&mut self, ctx: &CommunicationDescriptionEntryFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
		fn visit_communicationDescriptionEntryFormat3(&mut self, ctx: &CommunicationDescriptionEntryFormat3Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#destinationCountClause}.
	 * @param ctx the parse tree
	 */
		fn visit_destinationCountClause(&mut self, ctx: &DestinationCountClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#destinationTableClause}.
	 * @param ctx the parse tree
	 */
		fn visit_destinationTableClause(&mut self, ctx: &DestinationTableClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#endKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_endKeyClause(&mut self, ctx: &EndKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#errorKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_errorKeyClause(&mut self, ctx: &ErrorKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageCountClause}.
	 * @param ctx the parse tree
	 */
		fn visit_messageCountClause(&mut self, ctx: &MessageCountClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageDateClause}.
	 * @param ctx the parse tree
	 */
		fn visit_messageDateClause(&mut self, ctx: &MessageDateClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#messageTimeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_messageTimeClause(&mut self, ctx: &MessageTimeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#statusKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_statusKeyClause(&mut self, ctx: &StatusKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicDestinationClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicDestinationClause(&mut self, ctx: &SymbolicDestinationClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicQueueClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicQueueClause(&mut self, ctx: &SymbolicQueueClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicSourceClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicSourceClause(&mut self, ctx: &SymbolicSourceClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicTerminalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicTerminalClause(&mut self, ctx: &SymbolicTerminalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicSubQueueClause}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicSubQueueClause(&mut self, ctx: &SymbolicSubQueueClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#textLengthClause}.
	 * @param ctx the parse tree
	 */
		fn visit_textLengthClause(&mut self, ctx: &TextLengthClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#localStorageSection}.
	 * @param ctx the parse tree
	 */
		fn visit_localStorageSection(&mut self, ctx: &LocalStorageSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenSection}.
	 * @param ctx the parse tree
	 */
		fn visit_screenSection(&mut self, ctx: &ScreenSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionEntry(&mut self, ctx: &ScreenDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionBlankClause(&mut self, ctx: &ScreenDescriptionBlankClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBellClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionBellClause(&mut self, ctx: &ScreenDescriptionBellClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlinkClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionBlinkClause(&mut self, ctx: &ScreenDescriptionBlinkClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionEraseClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionEraseClause(&mut self, ctx: &ScreenDescriptionEraseClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionLightClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionLightClause(&mut self, ctx: &ScreenDescriptionLightClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionGridClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionGridClause(&mut self, ctx: &ScreenDescriptionGridClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionReverseVideoClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionReverseVideoClause(&mut self, ctx: &ScreenDescriptionReverseVideoClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUnderlineClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionUnderlineClause(&mut self, ctx: &ScreenDescriptionUnderlineClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSizeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionSizeClause(&mut self, ctx: &ScreenDescriptionSizeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionLineClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionLineClause(&mut self, ctx: &ScreenDescriptionLineClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionColumnClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionColumnClause(&mut self, ctx: &ScreenDescriptionColumnClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionForegroundColorClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionForegroundColorClause(&mut self, ctx: &ScreenDescriptionForegroundColorClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBackgroundColorClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionBackgroundColorClause(&mut self, ctx: &ScreenDescriptionBackgroundColorClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionControlClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionControlClause(&mut self, ctx: &ScreenDescriptionControlClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionValueClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionValueClause(&mut self, ctx: &ScreenDescriptionValueClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPictureClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionPictureClause(&mut self, ctx: &ScreenDescriptionPictureClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionFromClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionFromClause(&mut self, ctx: &ScreenDescriptionFromClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionToClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionToClause(&mut self, ctx: &ScreenDescriptionToClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionUsingClause(&mut self, ctx: &ScreenDescriptionUsingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsageClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionUsageClause(&mut self, ctx: &ScreenDescriptionUsageClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionBlankWhenZeroClause(&mut self, ctx: &ScreenDescriptionBlankWhenZeroClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionJustifiedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionJustifiedClause(&mut self, ctx: &ScreenDescriptionJustifiedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionSignClause(&mut self, ctx: &ScreenDescriptionSignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionAutoClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionAutoClause(&mut self, ctx: &ScreenDescriptionAutoClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionSecureClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionSecureClause(&mut self, ctx: &ScreenDescriptionSecureClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionRequiredClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionRequiredClause(&mut self, ctx: &ScreenDescriptionRequiredClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionPromptClause(&mut self, ctx: &ScreenDescriptionPromptClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptOccursClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionPromptOccursClause(&mut self, ctx: &ScreenDescriptionPromptOccursClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionFullClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionFullClause(&mut self, ctx: &ScreenDescriptionFullClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenDescriptionZeroFillClause}.
	 * @param ctx the parse tree
	 */
		fn visit_screenDescriptionZeroFillClause(&mut self, ctx: &ScreenDescriptionZeroFillClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportSection}.
	 * @param ctx the parse tree
	 */
		fn visit_reportSection(&mut self, ctx: &ReportSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescription}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescription(&mut self, ctx: &ReportDescriptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionEntry(&mut self, ctx: &ReportDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionGlobalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionGlobalClause(&mut self, ctx: &ReportDescriptionGlobalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionPageLimitClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionPageLimitClause(&mut self, ctx: &ReportDescriptionPageLimitClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionHeadingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionHeadingClause(&mut self, ctx: &ReportDescriptionHeadingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionFirstDetailClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionFirstDetailClause(&mut self, ctx: &ReportDescriptionFirstDetailClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionLastDetailClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionLastDetailClause(&mut self, ctx: &ReportDescriptionLastDetailClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportDescriptionFootingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportDescriptionFootingClause(&mut self, ctx: &ReportDescriptionFootingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupDescriptionEntry(&mut self, ctx: &ReportGroupDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupDescriptionEntryFormat1(&mut self, ctx: &ReportGroupDescriptionEntryFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupDescriptionEntryFormat2(&mut self, ctx: &ReportGroupDescriptionEntryFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupDescriptionEntryFormat3(&mut self, ctx: &ReportGroupDescriptionEntryFormat3Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupBlankWhenZeroClause(&mut self, ctx: &ReportGroupBlankWhenZeroClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupColumnNumberClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupColumnNumberClause(&mut self, ctx: &ReportGroupColumnNumberClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupIndicateClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupIndicateClause(&mut self, ctx: &ReportGroupIndicateClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupJustifiedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupJustifiedClause(&mut self, ctx: &ReportGroupJustifiedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupLineNumberClause(&mut self, ctx: &ReportGroupLineNumberClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberNextPage}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupLineNumberNextPage(&mut self, ctx: &ReportGroupLineNumberNextPageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberPlus}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupLineNumberPlus(&mut self, ctx: &ReportGroupLineNumberPlusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupNextGroupClause(&mut self, ctx: &ReportGroupNextGroupClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupPlus}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupNextGroupPlus(&mut self, ctx: &ReportGroupNextGroupPlusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupNextPage}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupNextGroupNextPage(&mut self, ctx: &ReportGroupNextGroupNextPageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupPictureClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupPictureClause(&mut self, ctx: &ReportGroupPictureClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupResetClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupResetClause(&mut self, ctx: &ReportGroupResetClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupSignClause(&mut self, ctx: &ReportGroupSignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSourceClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupSourceClause(&mut self, ctx: &ReportGroupSourceClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupSumClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupSumClause(&mut self, ctx: &ReportGroupSumClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeClause(&mut self, ctx: &ReportGroupTypeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportHeading}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeReportHeading(&mut self, ctx: &ReportGroupTypeReportHeadingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageHeading}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypePageHeading(&mut self, ctx: &ReportGroupTypePageHeadingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlHeading}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeControlHeading(&mut self, ctx: &ReportGroupTypeControlHeadingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeDetail}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeDetail(&mut self, ctx: &ReportGroupTypeDetailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlFooting}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeControlFooting(&mut self, ctx: &ReportGroupTypeControlFootingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupUsageClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupUsageClause(&mut self, ctx: &ReportGroupUsageClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageFooting}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypePageFooting(&mut self, ctx: &ReportGroupTypePageFootingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportFooting}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupTypeReportFooting(&mut self, ctx: &ReportGroupTypeReportFootingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportGroupValueClause}.
	 * @param ctx the parse tree
	 */
		fn visit_reportGroupValueClause(&mut self, ctx: &ReportGroupValueClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programLibrarySection}.
	 * @param ctx the parse tree
	 */
		fn visit_programLibrarySection(&mut self, ctx: &ProgramLibrarySectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryDescriptionEntry(&mut self, ctx: &LibraryDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryDescriptionEntryFormat1(&mut self, ctx: &LibraryDescriptionEntryFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryDescriptionEntryFormat2(&mut self, ctx: &LibraryDescriptionEntryFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryAttributeClauseFormat1(&mut self, ctx: &LibraryAttributeClauseFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryAttributeClauseFormat2(&mut self, ctx: &LibraryAttributeClauseFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryAttributeFunction(&mut self, ctx: &LibraryAttributeFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryAttributeParameter(&mut self, ctx: &LibraryAttributeParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryAttributeTitle}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryAttributeTitle(&mut self, ctx: &LibraryAttributeTitleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureClauseFormat1(&mut self, ctx: &LibraryEntryProcedureClauseFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureClauseFormat2(&mut self, ctx: &LibraryEntryProcedureClauseFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureForClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureForClause(&mut self, ctx: &LibraryEntryProcedureForClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureGivingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureGivingClause(&mut self, ctx: &LibraryEntryProcedureGivingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureUsingClause(&mut self, ctx: &LibraryEntryProcedureUsingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingName}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureUsingName(&mut self, ctx: &LibraryEntryProcedureUsingNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureWithClause(&mut self, ctx: &LibraryEntryProcedureWithClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithName}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryEntryProcedureWithName(&mut self, ctx: &LibraryEntryProcedureWithNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryIsCommonClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryIsCommonClause(&mut self, ctx: &LibraryIsCommonClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryIsGlobalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryIsGlobalClause(&mut self, ctx: &LibraryIsGlobalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescriptionEntry(&mut self, ctx: &DataDescriptionEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescriptionEntryFormat1(&mut self, ctx: &DataDescriptionEntryFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescriptionEntryFormat2(&mut self, ctx: &DataDescriptionEntryFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat3}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescriptionEntryFormat3(&mut self, ctx: &DataDescriptionEntryFormat3Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryExecSql}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescriptionEntryExecSql(&mut self, ctx: &DataDescriptionEntryExecSqlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataAlignedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataAlignedClause(&mut self, ctx: &DataAlignedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataBlankWhenZeroClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataBlankWhenZeroClause(&mut self, ctx: &DataBlankWhenZeroClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataCommonOwnLocalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataCommonOwnLocalClause(&mut self, ctx: &DataCommonOwnLocalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataExternalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataExternalClause(&mut self, ctx: &DataExternalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataGlobalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataGlobalClause(&mut self, ctx: &DataGlobalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataIntegerStringClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataIntegerStringClause(&mut self, ctx: &DataIntegerStringClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataJustifiedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataJustifiedClause(&mut self, ctx: &DataJustifiedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataOccursClause(&mut self, ctx: &DataOccursClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursTo}.
	 * @param ctx the parse tree
	 */
		fn visit_dataOccursTo(&mut self, ctx: &DataOccursToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataOccursSort}.
	 * @param ctx the parse tree
	 */
		fn visit_dataOccursSort(&mut self, ctx: &DataOccursSortContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataPictureClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataPictureClause(&mut self, ctx: &DataPictureClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureString}.
	 * @param ctx the parse tree
	 */
		fn visit_pictureString(&mut self, ctx: &PictureStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureChars}.
	 * @param ctx the parse tree
	 */
		fn visit_pictureChars(&mut self, ctx: &PictureCharsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#pictureCardinality}.
	 * @param ctx the parse tree
	 */
		fn visit_pictureCardinality(&mut self, ctx: &PictureCardinalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataReceivedByClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataReceivedByClause(&mut self, ctx: &DataReceivedByClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRecordAreaClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataRecordAreaClause(&mut self, ctx: &DataRecordAreaClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRedefinesClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataRedefinesClause(&mut self, ctx: &DataRedefinesClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataRenamesClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataRenamesClause(&mut self, ctx: &DataRenamesClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataSignClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataSignClause(&mut self, ctx: &DataSignClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataSynchronizedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataSynchronizedClause(&mut self, ctx: &DataSynchronizedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataThreadLocalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataThreadLocalClause(&mut self, ctx: &DataThreadLocalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataTypeClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataTypeClause(&mut self, ctx: &DataTypeClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataTypeDefClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataTypeDefClause(&mut self, ctx: &DataTypeDefClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataUsageClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataUsageClause(&mut self, ctx: &DataUsageClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataUsingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataUsingClause(&mut self, ctx: &DataUsingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataValueClause(&mut self, ctx: &DataValueClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_dataValueInterval(&mut self, ctx: &DataValueIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueIntervalFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_dataValueIntervalFrom(&mut self, ctx: &DataValueIntervalFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataValueIntervalTo}.
	 * @param ctx the parse tree
	 */
		fn visit_dataValueIntervalTo(&mut self, ctx: &DataValueIntervalToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataWithLowerBoundsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_dataWithLowerBoundsClause(&mut self, ctx: &DataWithLowerBoundsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivision}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivision(&mut self, ctx: &ProcedureDivisionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionUsingClause(&mut self, ctx: &ProcedureDivisionUsingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionGivingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionGivingClause(&mut self, ctx: &ProcedureDivisionGivingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionUsingParameter(&mut self, ctx: &ProcedureDivisionUsingParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReferencePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionByReferencePhrase(&mut self, ctx: &ProcedureDivisionByReferencePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReference}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionByReference(&mut self, ctx: &ProcedureDivisionByReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValuePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionByValuePhrase(&mut self, ctx: &ProcedureDivisionByValuePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValue}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionByValue(&mut self, ctx: &ProcedureDivisionByValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDeclaratives}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDeclaratives(&mut self, ctx: &ProcedureDeclarativesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDeclarative}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDeclarative(&mut self, ctx: &ProcedureDeclarativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureSectionHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureSectionHeader(&mut self, ctx: &ProcedureSectionHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureDivisionBody}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureDivisionBody(&mut self, ctx: &ProcedureDivisionBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureSection}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureSection(&mut self, ctx: &ProcedureSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraphs}.
	 * @param ctx the parse tree
	 */
		fn visit_paragraphs(&mut self, ctx: &ParagraphsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraph}.
	 * @param ctx the parse tree
	 */
		fn visit_paragraph(&mut self, ctx: &ParagraphContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sentence}.
	 * @param ctx the parse tree
	 */
		fn visit_sentence(&mut self, ctx: &SentenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_acceptStatement(&mut self, ctx: &AcceptStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromDateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_acceptFromDateStatement(&mut self, ctx: &AcceptFromDateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromMnemonicStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_acceptFromMnemonicStatement(&mut self, ctx: &AcceptFromMnemonicStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptFromEscapeKeyStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_acceptFromEscapeKeyStatement(&mut self, ctx: &AcceptFromEscapeKeyStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#acceptMessageCountStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_acceptMessageCountStatement(&mut self, ctx: &AcceptMessageCountStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addStatement(&mut self, ctx: &AddStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addToStatement(&mut self, ctx: &AddToStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToGivingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addToGivingStatement(&mut self, ctx: &AddToGivingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addCorrespondingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addCorrespondingStatement(&mut self, ctx: &AddCorrespondingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_addFrom(&mut self, ctx: &AddFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addTo}.
	 * @param ctx the parse tree
	 */
		fn visit_addTo(&mut self, ctx: &AddToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addToGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_addToGiving(&mut self, ctx: &AddToGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#addGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_addGiving(&mut self, ctx: &AddGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alteredGoTo}.
	 * @param ctx the parse tree
	 */
		fn visit_alteredGoTo(&mut self, ctx: &AlteredGoToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alterStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterStatement(&mut self, ctx: &AlterStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alterProceedTo}.
	 * @param ctx the parse tree
	 */
		fn visit_alterProceedTo(&mut self, ctx: &AlterProceedToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callUsingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_callUsingPhrase(&mut self, ctx: &CallUsingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callUsingParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_callUsingParameter(&mut self, ctx: &CallUsingParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByReferencePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_callByReferencePhrase(&mut self, ctx: &CallByReferencePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByReference}.
	 * @param ctx the parse tree
	 */
		fn visit_callByReference(&mut self, ctx: &CallByReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByValuePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_callByValuePhrase(&mut self, ctx: &CallByValuePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByValue}.
	 * @param ctx the parse tree
	 */
		fn visit_callByValue(&mut self, ctx: &CallByValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByContentPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_callByContentPhrase(&mut self, ctx: &CallByContentPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callByContent}.
	 * @param ctx the parse tree
	 */
		fn visit_callByContent(&mut self, ctx: &CallByContentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#callGivingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_callGivingPhrase(&mut self, ctx: &CallGivingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cancelStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_cancelStatement(&mut self, ctx: &CancelStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cancelCall}.
	 * @param ctx the parse tree
	 */
		fn visit_cancelCall(&mut self, ctx: &CancelCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_closeStatement(&mut self, ctx: &CloseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeFile}.
	 * @param ctx the parse tree
	 */
		fn visit_closeFile(&mut self, ctx: &CloseFileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeReelUnitStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_closeReelUnitStatement(&mut self, ctx: &CloseReelUnitStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closeRelativeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_closeRelativeStatement(&mut self, ctx: &CloseRelativeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_closePortFileIOStatement(&mut self, ctx: &ClosePortFileIOStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsing}.
	 * @param ctx the parse tree
	 */
		fn visit_closePortFileIOUsing(&mut self, ctx: &ClosePortFileIOUsingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingCloseDisposition}.
	 * @param ctx the parse tree
	 */
		fn visit_closePortFileIOUsingCloseDisposition(&mut self, ctx: &ClosePortFileIOUsingCloseDispositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedData}.
	 * @param ctx the parse tree
	 */
		fn visit_closePortFileIOUsingAssociatedData(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedDataLength}.
	 * @param ctx the parse tree
	 */
		fn visit_closePortFileIOUsingAssociatedDataLength(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataLengthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_computeStatement(&mut self, ctx: &ComputeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computeStore}.
	 * @param ctx the parse tree
	 */
		fn visit_computeStore(&mut self, ctx: &ComputeStoreContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#continueStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#deleteStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#disableStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_disableStatement(&mut self, ctx: &DisableStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_displayStatement(&mut self, ctx: &DisplayStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_displayOperand(&mut self, ctx: &DisplayOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayAt}.
	 * @param ctx the parse tree
	 */
		fn visit_displayAt(&mut self, ctx: &DisplayAtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayUpon}.
	 * @param ctx the parse tree
	 */
		fn visit_displayUpon(&mut self, ctx: &DisplayUponContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#displayWith}.
	 * @param ctx the parse tree
	 */
		fn visit_displayWith(&mut self, ctx: &DisplayWithContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_divideStatement(&mut self, ctx: &DivideStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideIntoStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_divideIntoStatement(&mut self, ctx: &DivideIntoStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideIntoGivingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_divideIntoGivingStatement(&mut self, ctx: &DivideIntoGivingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideByGivingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_divideByGivingStatement(&mut self, ctx: &DivideByGivingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideGivingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_divideGivingPhrase(&mut self, ctx: &DivideGivingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideInto}.
	 * @param ctx the parse tree
	 */
		fn visit_divideInto(&mut self, ctx: &DivideIntoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_divideGiving(&mut self, ctx: &DivideGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#divideRemainder}.
	 * @param ctx the parse tree
	 */
		fn visit_divideRemainder(&mut self, ctx: &DivideRemainderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#enableStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_enableStatement(&mut self, ctx: &EnableStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#entryStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_entryStatement(&mut self, ctx: &EntryStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateSelect}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateSelect(&mut self, ctx: &EvaluateSelectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateAlsoSelect}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateAlsoSelect(&mut self, ctx: &EvaluateAlsoSelectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhenPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateWhenPhrase(&mut self, ctx: &EvaluateWhenPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhen}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateWhen(&mut self, ctx: &EvaluateWhenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateCondition(&mut self, ctx: &EvaluateConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateThrough(&mut self, ctx: &EvaluateThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateAlsoCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateAlsoCondition(&mut self, ctx: &EvaluateAlsoConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateWhenOther}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateWhenOther(&mut self, ctx: &EvaluateWhenOtherContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#evaluateValue}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateValue(&mut self, ctx: &EvaluateValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execCicsStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execSqlStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#execSqlImsStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exhibitStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_exhibitStatement(&mut self, ctx: &ExhibitStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exhibitOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_exhibitOperand(&mut self, ctx: &ExhibitOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#exitStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_exitStatement(&mut self, ctx: &ExitStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#generateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_generateStatement(&mut self, ctx: &GenerateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#gobackStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_gobackStatement(&mut self, ctx: &GobackStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_goToStatement(&mut self, ctx: &GoToStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToStatementSimple}.
	 * @param ctx the parse tree
	 */
		fn visit_goToStatementSimple(&mut self, ctx: &GoToStatementSimpleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#goToDependingOnStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_goToDependingOnStatement(&mut self, ctx: &GoToDependingOnStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifThen}.
	 * @param ctx the parse tree
	 */
		fn visit_ifThen(&mut self, ctx: &IfThenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#ifElse}.
	 * @param ctx the parse tree
	 */
		fn visit_ifElse(&mut self, ctx: &IfElseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_initializeStatement(&mut self, ctx: &InitializeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeReplacingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_initializeReplacingPhrase(&mut self, ctx: &InitializeReplacingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initializeReplacingBy}.
	 * @param ctx the parse tree
	 */
		fn visit_initializeReplacingBy(&mut self, ctx: &InitializeReplacingByContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#initiateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_initiateStatement(&mut self, ctx: &InitiateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectStatement(&mut self, ctx: &InspectStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTallyingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectTallyingPhrase(&mut self, ctx: &InspectTallyingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectReplacingPhrase(&mut self, ctx: &InspectReplacingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTallyingReplacingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectTallyingReplacingPhrase(&mut self, ctx: &InspectTallyingReplacingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectConvertingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectConvertingPhrase(&mut self, ctx: &InspectConvertingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectFor}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectFor(&mut self, ctx: &InspectForContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectCharacters}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectCharacters(&mut self, ctx: &InspectCharactersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingCharacters}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectReplacingCharacters(&mut self, ctx: &InspectReplacingCharactersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectAllLeadings}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectAllLeadings(&mut self, ctx: &InspectAllLeadingsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeadings}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectReplacingAllLeadings(&mut self, ctx: &InspectReplacingAllLeadingsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectAllLeading}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectAllLeading(&mut self, ctx: &InspectAllLeadingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeading}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectReplacingAllLeading(&mut self, ctx: &InspectReplacingAllLeadingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectBy}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectBy(&mut self, ctx: &InspectByContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectTo}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectTo(&mut self, ctx: &InspectToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inspectBeforeAfter}.
	 * @param ctx the parse tree
	 */
		fn visit_inspectBeforeAfter(&mut self, ctx: &InspectBeforeAfterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeStatement(&mut self, ctx: &MergeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOnKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeOnKeyClause(&mut self, ctx: &MergeOnKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingSequencePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeCollatingSequencePhrase(&mut self, ctx: &MergeCollatingSequencePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingAlphanumeric}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeCollatingAlphanumeric(&mut self, ctx: &MergeCollatingAlphanumericContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeCollatingNational}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeCollatingNational(&mut self, ctx: &MergeCollatingNationalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeUsing}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeUsing(&mut self, ctx: &MergeUsingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOutputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeOutputProcedurePhrase(&mut self, ctx: &MergeOutputProcedurePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeOutputThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeOutputThrough(&mut self, ctx: &MergeOutputThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeGivingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeGivingPhrase(&mut self, ctx: &MergeGivingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mergeGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeGiving(&mut self, ctx: &MergeGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_moveStatement(&mut self, ctx: &MoveStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveToStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_moveToStatement(&mut self, ctx: &MoveToStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveToSendingArea}.
	 * @param ctx the parse tree
	 */
		fn visit_moveToSendingArea(&mut self, ctx: &MoveToSendingAreaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_moveCorrespondingToStatement(&mut self, ctx: &MoveCorrespondingToStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToSendingArea}.
	 * @param ctx the parse tree
	 */
		fn visit_moveCorrespondingToSendingArea(&mut self, ctx: &MoveCorrespondingToSendingAreaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyStatement(&mut self, ctx: &MultiplyStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyRegular}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyRegular(&mut self, ctx: &MultiplyRegularContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyRegularOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyRegularOperand(&mut self, ctx: &MultiplyRegularOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyGiving(&mut self, ctx: &MultiplyGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGivingOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyGivingOperand(&mut self, ctx: &MultiplyGivingOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multiplyGivingResult}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyGivingResult(&mut self, ctx: &MultiplyGivingResultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_openStatement(&mut self, ctx: &OpenStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openInputStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_openInputStatement(&mut self, ctx: &OpenInputStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openInput}.
	 * @param ctx the parse tree
	 */
		fn visit_openInput(&mut self, ctx: &OpenInputContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openOutputStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_openOutputStatement(&mut self, ctx: &OpenOutputStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openOutput}.
	 * @param ctx the parse tree
	 */
		fn visit_openOutput(&mut self, ctx: &OpenOutputContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openIOStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_openIOStatement(&mut self, ctx: &OpenIOStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#openExtendStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_openExtendStatement(&mut self, ctx: &OpenExtendStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_performStatement(&mut self, ctx: &PerformStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performInlineStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_performInlineStatement(&mut self, ctx: &PerformInlineStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performProcedureStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_performProcedureStatement(&mut self, ctx: &PerformProcedureStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performType}.
	 * @param ctx the parse tree
	 */
		fn visit_performType(&mut self, ctx: &PerformTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performTimes}.
	 * @param ctx the parse tree
	 */
		fn visit_performTimes(&mut self, ctx: &PerformTimesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performUntil}.
	 * @param ctx the parse tree
	 */
		fn visit_performUntil(&mut self, ctx: &PerformUntilContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVarying}.
	 * @param ctx the parse tree
	 */
		fn visit_performVarying(&mut self, ctx: &PerformVaryingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVaryingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_performVaryingClause(&mut self, ctx: &PerformVaryingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performVaryingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_performVaryingPhrase(&mut self, ctx: &PerformVaryingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performAfter}.
	 * @param ctx the parse tree
	 */
		fn visit_performAfter(&mut self, ctx: &PerformAfterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_performFrom(&mut self, ctx: &PerformFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performBy}.
	 * @param ctx the parse tree
	 */
		fn visit_performBy(&mut self, ctx: &PerformByContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#performTestClause}.
	 * @param ctx the parse tree
	 */
		fn visit_performTestClause(&mut self, ctx: &PerformTestClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#purgeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_purgeStatement(&mut self, ctx: &PurgeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_readStatement(&mut self, ctx: &ReadStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readInto}.
	 * @param ctx the parse tree
	 */
		fn visit_readInto(&mut self, ctx: &ReadIntoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readWith}.
	 * @param ctx the parse tree
	 */
		fn visit_readWith(&mut self, ctx: &ReadWithContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#readKey}.
	 * @param ctx the parse tree
	 */
		fn visit_readKey(&mut self, ctx: &ReadKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveStatement(&mut self, ctx: &ReceiveStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveFromStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveFromStatement(&mut self, ctx: &ReceiveFromStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveFrom(&mut self, ctx: &ReceiveFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveIntoStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveIntoStatement(&mut self, ctx: &ReceiveIntoStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveNoData}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveNoData(&mut self, ctx: &ReceiveNoDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveWithData}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveWithData(&mut self, ctx: &ReceiveWithDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveBefore}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveBefore(&mut self, ctx: &ReceiveBeforeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveWith}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveWith(&mut self, ctx: &ReceiveWithContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveThread}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveThread(&mut self, ctx: &ReceiveThreadContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveSize}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveSize(&mut self, ctx: &ReceiveSizeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#receiveStatus}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveStatus(&mut self, ctx: &ReceiveStatusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#releaseStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_releaseStatement(&mut self, ctx: &ReleaseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#returnStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#returnInto}.
	 * @param ctx the parse tree
	 */
		fn visit_returnInto(&mut self, ctx: &ReturnIntoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rewriteStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_rewriteStatement(&mut self, ctx: &RewriteStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#rewriteFrom}.
	 * @param ctx the parse tree
	 */
		fn visit_rewriteFrom(&mut self, ctx: &RewriteFromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_searchStatement(&mut self, ctx: &SearchStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchVarying}.
	 * @param ctx the parse tree
	 */
		fn visit_searchVarying(&mut self, ctx: &SearchVaryingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#searchWhen}.
	 * @param ctx the parse tree
	 */
		fn visit_searchWhen(&mut self, ctx: &SearchWhenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_sendStatement(&mut self, ctx: &SendStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatementSync}.
	 * @param ctx the parse tree
	 */
		fn visit_sendStatementSync(&mut self, ctx: &SendStatementSyncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendStatementAsync}.
	 * @param ctx the parse tree
	 */
		fn visit_sendStatementAsync(&mut self, ctx: &SendStatementAsyncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendFromPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sendFromPhrase(&mut self, ctx: &SendFromPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendWithPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sendWithPhrase(&mut self, ctx: &SendWithPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendReplacingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sendReplacingPhrase(&mut self, ctx: &SendReplacingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sendAdvancingPhrase(&mut self, ctx: &SendAdvancingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingPage}.
	 * @param ctx the parse tree
	 */
		fn visit_sendAdvancingPage(&mut self, ctx: &SendAdvancingPageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingLines}.
	 * @param ctx the parse tree
	 */
		fn visit_sendAdvancingLines(&mut self, ctx: &SendAdvancingLinesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sendAdvancingMnemonic}.
	 * @param ctx the parse tree
	 */
		fn visit_sendAdvancingMnemonic(&mut self, ctx: &SendAdvancingMnemonicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setToStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setToStatement(&mut self, ctx: &SetToStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setUpDownByStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setUpDownByStatement(&mut self, ctx: &SetUpDownByStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setTo}.
	 * @param ctx the parse tree
	 */
		fn visit_setTo(&mut self, ctx: &SetToContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setToValue}.
	 * @param ctx the parse tree
	 */
		fn visit_setToValue(&mut self, ctx: &SetToValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#setByValue}.
	 * @param ctx the parse tree
	 */
		fn visit_setByValue(&mut self, ctx: &SetByValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_sortStatement(&mut self, ctx: &SortStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOnKeyClause}.
	 * @param ctx the parse tree
	 */
		fn visit_sortOnKeyClause(&mut self, ctx: &SortOnKeyClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortDuplicatesPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sortDuplicatesPhrase(&mut self, ctx: &SortDuplicatesPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingSequencePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sortCollatingSequencePhrase(&mut self, ctx: &SortCollatingSequencePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingAlphanumeric}.
	 * @param ctx the parse tree
	 */
		fn visit_sortCollatingAlphanumeric(&mut self, ctx: &SortCollatingAlphanumericContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortCollatingNational}.
	 * @param ctx the parse tree
	 */
		fn visit_sortCollatingNational(&mut self, ctx: &SortCollatingNationalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortInputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sortInputProcedurePhrase(&mut self, ctx: &SortInputProcedurePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortInputThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_sortInputThrough(&mut self, ctx: &SortInputThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortUsing}.
	 * @param ctx the parse tree
	 */
		fn visit_sortUsing(&mut self, ctx: &SortUsingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOutputProcedurePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sortOutputProcedurePhrase(&mut self, ctx: &SortOutputProcedurePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortOutputThrough}.
	 * @param ctx the parse tree
	 */
		fn visit_sortOutputThrough(&mut self, ctx: &SortOutputThroughContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortGivingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_sortGivingPhrase(&mut self, ctx: &SortGivingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sortGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_sortGiving(&mut self, ctx: &SortGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_startStatement(&mut self, ctx: &StartStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#startKey}.
	 * @param ctx the parse tree
	 */
		fn visit_startKey(&mut self, ctx: &StartKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stopStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_stopStatement(&mut self, ctx: &StopStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringStatement(&mut self, ctx: &StringStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringSendingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_stringSendingPhrase(&mut self, ctx: &StringSendingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringSending}.
	 * @param ctx the parse tree
	 */
		fn visit_stringSending(&mut self, ctx: &StringSendingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringDelimitedByPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_stringDelimitedByPhrase(&mut self, ctx: &StringDelimitedByPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringForPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_stringForPhrase(&mut self, ctx: &StringForPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringIntoPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_stringIntoPhrase(&mut self, ctx: &StringIntoPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#stringWithPointerPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_stringWithPointerPhrase(&mut self, ctx: &StringWithPointerPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractStatement(&mut self, ctx: &SubtractStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractFromStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractFromStatement(&mut self, ctx: &SubtractFromStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractFromGivingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractFromGivingStatement(&mut self, ctx: &SubtractFromGivingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractCorrespondingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractCorrespondingStatement(&mut self, ctx: &SubtractCorrespondingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractSubtrahend}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractSubtrahend(&mut self, ctx: &SubtractSubtrahendContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuend}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractMinuend(&mut self, ctx: &SubtractMinuendContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuendGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractMinuendGiving(&mut self, ctx: &SubtractMinuendGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractGiving}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractGiving(&mut self, ctx: &SubtractGivingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subtractMinuendCorresponding}.
	 * @param ctx the parse tree
	 */
		fn visit_subtractMinuendCorresponding(&mut self, ctx: &SubtractMinuendCorrespondingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#terminateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_terminateStatement(&mut self, ctx: &TerminateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringStatement(&mut self, ctx: &UnstringStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringSendingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringSendingPhrase(&mut self, ctx: &UnstringSendingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringDelimitedByPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringDelimitedByPhrase(&mut self, ctx: &UnstringDelimitedByPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringOrAllPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringOrAllPhrase(&mut self, ctx: &UnstringOrAllPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringIntoPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringIntoPhrase(&mut self, ctx: &UnstringIntoPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringInto}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringInto(&mut self, ctx: &UnstringIntoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringDelimiterIn}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringDelimiterIn(&mut self, ctx: &UnstringDelimiterInContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringCountIn}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringCountIn(&mut self, ctx: &UnstringCountInContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringWithPointerPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringWithPointerPhrase(&mut self, ctx: &UnstringWithPointerPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#unstringTallyingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_unstringTallyingPhrase(&mut self, ctx: &UnstringTallyingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_useStatement(&mut self, ctx: &UseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useAfterClause}.
	 * @param ctx the parse tree
	 */
		fn visit_useAfterClause(&mut self, ctx: &UseAfterClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useAfterOn}.
	 * @param ctx the parse tree
	 */
		fn visit_useAfterOn(&mut self, ctx: &UseAfterOnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useDebugClause}.
	 * @param ctx the parse tree
	 */
		fn visit_useDebugClause(&mut self, ctx: &UseDebugClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#useDebugOn}.
	 * @param ctx the parse tree
	 */
		fn visit_useDebugOn(&mut self, ctx: &UseDebugOnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_writeStatement(&mut self, ctx: &WriteStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeFromPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_writeFromPhrase(&mut self, ctx: &WriteFromPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_writeAdvancingPhrase(&mut self, ctx: &WriteAdvancingPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingPage}.
	 * @param ctx the parse tree
	 */
		fn visit_writeAdvancingPage(&mut self, ctx: &WriteAdvancingPageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingLines}.
	 * @param ctx the parse tree
	 */
		fn visit_writeAdvancingLines(&mut self, ctx: &WriteAdvancingLinesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAdvancingMnemonic}.
	 * @param ctx the parse tree
	 */
		fn visit_writeAdvancingMnemonic(&mut self, ctx: &WriteAdvancingMnemonicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeAtEndOfPagePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_writeAtEndOfPagePhrase(&mut self, ctx: &WriteAtEndOfPagePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#writeNotAtEndOfPagePhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_writeNotAtEndOfPagePhrase(&mut self, ctx: &WriteNotAtEndOfPagePhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#atEndPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_atEndPhrase(&mut self, ctx: &AtEndPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notAtEndPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_notAtEndPhrase(&mut self, ctx: &NotAtEndPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#invalidKeyPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_invalidKeyPhrase(&mut self, ctx: &InvalidKeyPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notInvalidKeyPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_notInvalidKeyPhrase(&mut self, ctx: &NotInvalidKeyPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onOverflowPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_onOverflowPhrase(&mut self, ctx: &OnOverflowPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnOverflowPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_notOnOverflowPhrase(&mut self, ctx: &NotOnOverflowPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onSizeErrorPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_onSizeErrorPhrase(&mut self, ctx: &OnSizeErrorPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnSizeErrorPhrase}.
	 * @param ctx the parse tree
	 */
		fn visit_notOnSizeErrorPhrase(&mut self, ctx: &NotOnSizeErrorPhraseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#onExceptionClause}.
	 * @param ctx the parse tree
	 */
		fn visit_onExceptionClause(&mut self, ctx: &OnExceptionClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#notOnExceptionClause}.
	 * @param ctx the parse tree
	 */
		fn visit_notOnExceptionClause(&mut self, ctx: &NotOnExceptionClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#arithmeticExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticExpression(&mut self, ctx: &ArithmeticExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#plusMinus}.
	 * @param ctx the parse tree
	 */
		fn visit_plusMinus(&mut self, ctx: &PlusMinusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multDivs}.
	 * @param ctx the parse tree
	 */
		fn visit_multDivs(&mut self, ctx: &MultDivsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#multDiv}.
	 * @param ctx the parse tree
	 */
		fn visit_multDiv(&mut self, ctx: &MultDivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#powers}.
	 * @param ctx the parse tree
	 */
		fn visit_powers(&mut self, ctx: &PowersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#power}.
	 * @param ctx the parse tree
	 */
		fn visit_power(&mut self, ctx: &PowerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#basis}.
	 * @param ctx the parse tree
	 */
		fn visit_basis(&mut self, ctx: &BasisContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#condition}.
	 * @param ctx the parse tree
	 */
		fn visit_condition(&mut self, ctx: &ConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#andOrCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_andOrCondition(&mut self, ctx: &AndOrConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#combinableCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_combinableCondition(&mut self, ctx: &CombinableConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#simpleCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleCondition(&mut self, ctx: &SimpleConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#classCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_classCondition(&mut self, ctx: &ClassConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionNameReference}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionNameReference(&mut self, ctx: &ConditionNameReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionNameSubscriptReference}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionNameSubscriptReference(&mut self, ctx: &ConditionNameSubscriptReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_relationCondition(&mut self, ctx: &RelationConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationSignCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_relationSignCondition(&mut self, ctx: &RelationSignConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationArithmeticComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_relationArithmeticComparison(&mut self, ctx: &RelationArithmeticComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCombinedComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_relationCombinedComparison(&mut self, ctx: &RelationCombinedComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationCombinedCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_relationCombinedCondition(&mut self, ctx: &RelationCombinedConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#relationalOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#abbreviation}.
	 * @param ctx the parse tree
	 */
		fn visit_abbreviation(&mut self, ctx: &AbbreviationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#tableCall}.
	 * @param ctx the parse tree
	 */
		fn visit_tableCall(&mut self, ctx: &TableCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#functionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#referenceModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_referenceModifier(&mut self, ctx: &ReferenceModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#characterPosition}.
	 * @param ctx the parse tree
	 */
		fn visit_characterPosition(&mut self, ctx: &CharacterPositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#length}.
	 * @param ctx the parse tree
	 */
		fn visit_length(&mut self, ctx: &LengthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#subscript_}.
	 * @param ctx the parse tree
	 */
		fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#argument}.
	 * @param ctx the parse tree
	 */
		fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataName}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedDataName(&mut self, ctx: &QualifiedDataNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat1}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedDataNameFormat1(&mut self, ctx: &QualifiedDataNameFormat1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat2}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedDataNameFormat2(&mut self, ctx: &QualifiedDataNameFormat2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat3}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedDataNameFormat3(&mut self, ctx: &QualifiedDataNameFormat3Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat4}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedDataNameFormat4(&mut self, ctx: &QualifiedDataNameFormat4Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#qualifiedInData}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedInData(&mut self, ctx: &QualifiedInDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inData}.
	 * @param ctx the parse tree
	 */
		fn visit_inData(&mut self, ctx: &InDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inFile}.
	 * @param ctx the parse tree
	 */
		fn visit_inFile(&mut self, ctx: &InFileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inMnemonic}.
	 * @param ctx the parse tree
	 */
		fn visit_inMnemonic(&mut self, ctx: &InMnemonicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inSection}.
	 * @param ctx the parse tree
	 */
		fn visit_inSection(&mut self, ctx: &InSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inLibrary}.
	 * @param ctx the parse tree
	 */
		fn visit_inLibrary(&mut self, ctx: &InLibraryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#inTable}.
	 * @param ctx the parse tree
	 */
		fn visit_inTable(&mut self, ctx: &InTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#alphabetName}.
	 * @param ctx the parse tree
	 */
		fn visit_alphabetName(&mut self, ctx: &AlphabetNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#assignmentName}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentName(&mut self, ctx: &AssignmentNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#basisName}.
	 * @param ctx the parse tree
	 */
		fn visit_basisName(&mut self, ctx: &BasisNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cdName}.
	 * @param ctx the parse tree
	 */
		fn visit_cdName(&mut self, ctx: &CdNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#className}.
	 * @param ctx the parse tree
	 */
		fn visit_className(&mut self, ctx: &ClassNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#computerName}.
	 * @param ctx the parse tree
	 */
		fn visit_computerName(&mut self, ctx: &ComputerNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#conditionName}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionName(&mut self, ctx: &ConditionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataName}.
	 * @param ctx the parse tree
	 */
		fn visit_dataName(&mut self, ctx: &DataNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#dataDescName}.
	 * @param ctx the parse tree
	 */
		fn visit_dataDescName(&mut self, ctx: &DataDescNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#environmentName}.
	 * @param ctx the parse tree
	 */
		fn visit_environmentName(&mut self, ctx: &EnvironmentNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#fileName}.
	 * @param ctx the parse tree
	 */
		fn visit_fileName(&mut self, ctx: &FileNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#functionName}.
	 * @param ctx the parse tree
	 */
		fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#indexName}.
	 * @param ctx the parse tree
	 */
		fn visit_indexName(&mut self, ctx: &IndexNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#languageName}.
	 * @param ctx the parse tree
	 */
		fn visit_languageName(&mut self, ctx: &LanguageNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#libraryName}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryName(&mut self, ctx: &LibraryNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#localName}.
	 * @param ctx the parse tree
	 */
		fn visit_localName(&mut self, ctx: &LocalNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#mnemonicName}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonicName(&mut self, ctx: &MnemonicNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#paragraphName}.
	 * @param ctx the parse tree
	 */
		fn visit_paragraphName(&mut self, ctx: &ParagraphNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#procedureName}.
	 * @param ctx the parse tree
	 */
		fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#programName}.
	 * @param ctx the parse tree
	 */
		fn visit_programName(&mut self, ctx: &ProgramNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#recordName}.
	 * @param ctx the parse tree
	 */
		fn visit_recordName(&mut self, ctx: &RecordNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#reportName}.
	 * @param ctx the parse tree
	 */
		fn visit_reportName(&mut self, ctx: &ReportNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#routineName}.
	 * @param ctx the parse tree
	 */
		fn visit_routineName(&mut self, ctx: &RoutineNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#screenName}.
	 * @param ctx the parse tree
	 */
		fn visit_screenName(&mut self, ctx: &ScreenNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#sectionName}.
	 * @param ctx the parse tree
	 */
		fn visit_sectionName(&mut self, ctx: &SectionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#systemName}.
	 * @param ctx the parse tree
	 */
		fn visit_systemName(&mut self, ctx: &SystemNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#symbolicCharacter}.
	 * @param ctx the parse tree
	 */
		fn visit_symbolicCharacter(&mut self, ctx: &SymbolicCharacterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#textName}.
	 * @param ctx the parse tree
	 */
		fn visit_textName(&mut self, ctx: &TextNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cobolWord}.
	 * @param ctx the parse tree
	 */
		fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#numericLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#integerLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cicsDfhRespLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_cicsDfhRespLiteral(&mut self, ctx: &CicsDfhRespLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#cicsDfhValueLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_cicsDfhValueLiteral(&mut self, ctx: &CicsDfhValueLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#figurativeConstant}.
	 * @param ctx the parse tree
	 */
		fn visit_figurativeConstant(&mut self, ctx: &FigurativeConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#specialRegister}.
	 * @param ctx the parse tree
	 */
		fn visit_specialRegister(&mut self, ctx: &SpecialRegisterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link Cobol85Parser#commentEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_commentEntry(&mut self, ctx: &CommentEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> Cobol85Visitor<'input> for T
where
	T: Cobol85VisitorCompat<'input>
{
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_startRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_compilationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_programUnit(&mut self, ctx: &ProgramUnitContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_programUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endProgramStatement(&mut self, ctx: &EndProgramStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_endProgramStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identificationDivision(&mut self, ctx: &IdentificationDivisionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_identificationDivision(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identificationDivisionBody(&mut self, ctx: &IdentificationDivisionBodyContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_identificationDivisionBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_programIdParagraph(&mut self, ctx: &ProgramIdParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_programIdParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authorParagraph(&mut self, ctx: &AuthorParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_authorParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_installationParagraph(&mut self, ctx: &InstallationParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_installationParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dateWrittenParagraph(&mut self, ctx: &DateWrittenParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dateWrittenParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dateCompiledParagraph(&mut self, ctx: &DateCompiledParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dateCompiledParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_securityParagraph(&mut self, ctx: &SecurityParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_securityParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_remarksParagraph(&mut self, ctx: &RemarksParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_remarksParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_environmentDivision(&mut self, ctx: &EnvironmentDivisionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_environmentDivision(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_environmentDivisionBody(&mut self, ctx: &EnvironmentDivisionBodyContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_environmentDivisionBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configurationSection(&mut self, ctx: &ConfigurationSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_configurationSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configurationSectionParagraph(&mut self, ctx: &ConfigurationSectionParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_configurationSectionParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sourceComputerParagraph(&mut self, ctx: &SourceComputerParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sourceComputerParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectComputerParagraph(&mut self, ctx: &ObjectComputerParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_objectComputerParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectComputerClause(&mut self, ctx: &ObjectComputerClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_objectComputerClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memorySizeClause(&mut self, ctx: &MemorySizeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_memorySizeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_diskSizeClause(&mut self, ctx: &DiskSizeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_diskSizeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collatingSequenceClause(&mut self, ctx: &CollatingSequenceClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_collatingSequenceClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collatingSequenceClauseAlphanumeric(&mut self, ctx: &CollatingSequenceClauseAlphanumericContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_collatingSequenceClauseAlphanumeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collatingSequenceClauseNational(&mut self, ctx: &CollatingSequenceClauseNationalContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_collatingSequenceClauseNational(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_segmentLimitClause(&mut self, ctx: &SegmentLimitClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_segmentLimitClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_characterSetClause(&mut self, ctx: &CharacterSetClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_characterSetClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_specialNamesParagraph(&mut self, ctx: &SpecialNamesParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_specialNamesParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_specialNameClause(&mut self, ctx: &SpecialNameClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_specialNameClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetClause(&mut self, ctx: &AlphabetClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetClauseFormat1(&mut self, ctx: &AlphabetClauseFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetClauseFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetLiterals(&mut self, ctx: &AlphabetLiteralsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetLiterals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetThrough(&mut self, ctx: &AlphabetThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetAlso(&mut self, ctx: &AlphabetAlsoContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetAlso(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetClauseFormat2(&mut self, ctx: &AlphabetClauseFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetClauseFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_channelClause(&mut self, ctx: &ChannelClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_channelClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classClause(&mut self, ctx: &ClassClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_classClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classClauseThrough(&mut self, ctx: &ClassClauseThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_classClauseThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classClauseFrom(&mut self, ctx: &ClassClauseFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_classClauseFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classClauseTo(&mut self, ctx: &ClassClauseToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_classClauseTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_currencySignClause(&mut self, ctx: &CurrencySignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_currencySignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decimalPointClause(&mut self, ctx: &DecimalPointClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_decimalPointClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defaultComputationalSignClause(&mut self, ctx: &DefaultComputationalSignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_defaultComputationalSignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defaultDisplaySignClause(&mut self, ctx: &DefaultDisplaySignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_defaultDisplaySignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_environmentSwitchNameClause(&mut self, ctx: &EnvironmentSwitchNameClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_environmentSwitchNameClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_environmentSwitchNameSpecialNamesStatusPhrase(&mut self, ctx: &EnvironmentSwitchNameSpecialNamesStatusPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_environmentSwitchNameSpecialNamesStatusPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_odtClause(&mut self, ctx: &OdtClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_odtClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reserveNetworkClause(&mut self, ctx: &ReserveNetworkClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reserveNetworkClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicCharactersClause(&mut self, ctx: &SymbolicCharactersClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicCharactersClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicCharacters(&mut self, ctx: &SymbolicCharactersContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicCharacters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputOutputSection(&mut self, ctx: &InputOutputSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inputOutputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputOutputSectionParagraph(&mut self, ctx: &InputOutputSectionParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inputOutputSectionParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileControlParagraph(&mut self, ctx: &FileControlParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileControlParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileControlEntry(&mut self, ctx: &FileControlEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileControlEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_selectClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileControlClause(&mut self, ctx: &FileControlClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileControlClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignClause(&mut self, ctx: &AssignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_assignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reserveClause(&mut self, ctx: &ReserveClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reserveClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_organizationClause(&mut self, ctx: &OrganizationClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_organizationClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paddingCharacterClause(&mut self, ctx: &PaddingCharacterClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_paddingCharacterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordDelimiterClause(&mut self, ctx: &RecordDelimiterClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordDelimiterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_accessModeClause(&mut self, ctx: &AccessModeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_accessModeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordKeyClause(&mut self, ctx: &RecordKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alternateRecordKeyClause(&mut self, ctx: &AlternateRecordKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alternateRecordKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_passwordClause(&mut self, ctx: &PasswordClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_passwordClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileStatusClause(&mut self, ctx: &FileStatusClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileStatusClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relativeKeyClause(&mut self, ctx: &RelativeKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relativeKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ioControlParagraph(&mut self, ctx: &IoControlParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_ioControlParagraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ioControlClause(&mut self, ctx: &IoControlClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_ioControlClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rerunClause(&mut self, ctx: &RerunClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rerunClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rerunEveryRecords(&mut self, ctx: &RerunEveryRecordsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rerunEveryRecords(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rerunEveryOf(&mut self, ctx: &RerunEveryOfContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rerunEveryOf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rerunEveryClock(&mut self, ctx: &RerunEveryClockContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rerunEveryClock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sameClause(&mut self, ctx: &SameClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sameClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipleFileClause(&mut self, ctx: &MultipleFileClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multipleFileClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipleFilePosition(&mut self, ctx: &MultipleFilePositionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multipleFilePosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commitmentControlClause(&mut self, ctx: &CommitmentControlClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_commitmentControlClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDivision(&mut self, ctx: &DataDivisionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDivision(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDivisionSection(&mut self, ctx: &DataDivisionSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDivisionSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileSection(&mut self, ctx: &FileSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileDescriptionEntry(&mut self, ctx: &FileDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileDescriptionEntryClause(&mut self, ctx: &FileDescriptionEntryClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileDescriptionEntryClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externalClause(&mut self, ctx: &ExternalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_externalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_globalClause(&mut self, ctx: &GlobalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_globalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockContainsClause(&mut self, ctx: &BlockContainsClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_blockContainsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockContainsTo(&mut self, ctx: &BlockContainsToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_blockContainsTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordContainsClause(&mut self, ctx: &RecordContainsClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordContainsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordContainsClauseFormat1(&mut self, ctx: &RecordContainsClauseFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordContainsClauseFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordContainsClauseFormat2(&mut self, ctx: &RecordContainsClauseFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordContainsClauseFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordContainsClauseFormat3(&mut self, ctx: &RecordContainsClauseFormat3Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordContainsClauseFormat3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordContainsTo(&mut self, ctx: &RecordContainsToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordContainsTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_labelRecordsClause(&mut self, ctx: &LabelRecordsClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_labelRecordsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueOfClause(&mut self, ctx: &ValueOfClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_valueOfClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valuePair(&mut self, ctx: &ValuePairContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_valuePair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataRecordsClause(&mut self, ctx: &DataRecordsClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataRecordsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linageClause(&mut self, ctx: &LinageClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linageClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linageAt(&mut self, ctx: &LinageAtContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linageAt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linageFootingAt(&mut self, ctx: &LinageFootingAtContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linageFootingAt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linageLinesAtTop(&mut self, ctx: &LinageLinesAtTopContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linageLinesAtTop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linageLinesAtBottom(&mut self, ctx: &LinageLinesAtBottomContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linageLinesAtBottom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordingModeClause(&mut self, ctx: &RecordingModeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordingModeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modeStatement(&mut self, ctx: &ModeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_modeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_codeSetClause(&mut self, ctx: &CodeSetClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_codeSetClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportClause(&mut self, ctx: &ReportClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataBaseSection(&mut self, ctx: &DataBaseSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataBaseSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataBaseSectionEntry(&mut self, ctx: &DataBaseSectionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataBaseSectionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workingStorageSection(&mut self, ctx: &WorkingStorageSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_workingStorageSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linkageSection(&mut self, ctx: &LinkageSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_linkageSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_communicationSection(&mut self, ctx: &CommunicationSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_communicationSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_communicationDescriptionEntry(&mut self, ctx: &CommunicationDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_communicationDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_communicationDescriptionEntryFormat1(&mut self, ctx: &CommunicationDescriptionEntryFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_communicationDescriptionEntryFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_communicationDescriptionEntryFormat2(&mut self, ctx: &CommunicationDescriptionEntryFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_communicationDescriptionEntryFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_communicationDescriptionEntryFormat3(&mut self, ctx: &CommunicationDescriptionEntryFormat3Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_communicationDescriptionEntryFormat3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_destinationCountClause(&mut self, ctx: &DestinationCountClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_destinationCountClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_destinationTableClause(&mut self, ctx: &DestinationTableClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_destinationTableClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endKeyClause(&mut self, ctx: &EndKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_endKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorKeyClause(&mut self, ctx: &ErrorKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_errorKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_messageCountClause(&mut self, ctx: &MessageCountClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_messageCountClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_messageDateClause(&mut self, ctx: &MessageDateClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_messageDateClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_messageTimeClause(&mut self, ctx: &MessageTimeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_messageTimeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statusKeyClause(&mut self, ctx: &StatusKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_statusKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicDestinationClause(&mut self, ctx: &SymbolicDestinationClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicDestinationClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicQueueClause(&mut self, ctx: &SymbolicQueueClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicQueueClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicSourceClause(&mut self, ctx: &SymbolicSourceClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicSourceClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicTerminalClause(&mut self, ctx: &SymbolicTerminalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicTerminalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicSubQueueClause(&mut self, ctx: &SymbolicSubQueueClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicSubQueueClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_textLengthClause(&mut self, ctx: &TextLengthClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_textLengthClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localStorageSection(&mut self, ctx: &LocalStorageSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_localStorageSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenSection(&mut self, ctx: &ScreenSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionEntry(&mut self, ctx: &ScreenDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionBlankClause(&mut self, ctx: &ScreenDescriptionBlankClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionBlankClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionBellClause(&mut self, ctx: &ScreenDescriptionBellClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionBellClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionBlinkClause(&mut self, ctx: &ScreenDescriptionBlinkClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionBlinkClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionEraseClause(&mut self, ctx: &ScreenDescriptionEraseClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionEraseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionLightClause(&mut self, ctx: &ScreenDescriptionLightClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionLightClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionGridClause(&mut self, ctx: &ScreenDescriptionGridClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionGridClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionReverseVideoClause(&mut self, ctx: &ScreenDescriptionReverseVideoClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionReverseVideoClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionUnderlineClause(&mut self, ctx: &ScreenDescriptionUnderlineClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionUnderlineClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionSizeClause(&mut self, ctx: &ScreenDescriptionSizeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionSizeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionLineClause(&mut self, ctx: &ScreenDescriptionLineClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionLineClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionColumnClause(&mut self, ctx: &ScreenDescriptionColumnClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionColumnClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionForegroundColorClause(&mut self, ctx: &ScreenDescriptionForegroundColorClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionForegroundColorClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionBackgroundColorClause(&mut self, ctx: &ScreenDescriptionBackgroundColorClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionBackgroundColorClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionControlClause(&mut self, ctx: &ScreenDescriptionControlClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionControlClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionValueClause(&mut self, ctx: &ScreenDescriptionValueClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionValueClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionPictureClause(&mut self, ctx: &ScreenDescriptionPictureClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionPictureClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionFromClause(&mut self, ctx: &ScreenDescriptionFromClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionFromClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionToClause(&mut self, ctx: &ScreenDescriptionToClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionToClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionUsingClause(&mut self, ctx: &ScreenDescriptionUsingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionUsingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionUsageClause(&mut self, ctx: &ScreenDescriptionUsageClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionUsageClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionBlankWhenZeroClause(&mut self, ctx: &ScreenDescriptionBlankWhenZeroClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionBlankWhenZeroClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionJustifiedClause(&mut self, ctx: &ScreenDescriptionJustifiedClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionJustifiedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionSignClause(&mut self, ctx: &ScreenDescriptionSignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionSignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionAutoClause(&mut self, ctx: &ScreenDescriptionAutoClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionAutoClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionSecureClause(&mut self, ctx: &ScreenDescriptionSecureClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionSecureClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionRequiredClause(&mut self, ctx: &ScreenDescriptionRequiredClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionRequiredClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionPromptClause(&mut self, ctx: &ScreenDescriptionPromptClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionPromptClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionPromptOccursClause(&mut self, ctx: &ScreenDescriptionPromptOccursClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionPromptOccursClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionFullClause(&mut self, ctx: &ScreenDescriptionFullClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionFullClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenDescriptionZeroFillClause(&mut self, ctx: &ScreenDescriptionZeroFillClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenDescriptionZeroFillClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportSection(&mut self, ctx: &ReportSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescription(&mut self, ctx: &ReportDescriptionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescription(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionEntry(&mut self, ctx: &ReportDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionGlobalClause(&mut self, ctx: &ReportDescriptionGlobalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionGlobalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionPageLimitClause(&mut self, ctx: &ReportDescriptionPageLimitClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionPageLimitClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionHeadingClause(&mut self, ctx: &ReportDescriptionHeadingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionHeadingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionFirstDetailClause(&mut self, ctx: &ReportDescriptionFirstDetailClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionFirstDetailClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionLastDetailClause(&mut self, ctx: &ReportDescriptionLastDetailClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionLastDetailClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportDescriptionFootingClause(&mut self, ctx: &ReportDescriptionFootingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportDescriptionFootingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupDescriptionEntry(&mut self, ctx: &ReportGroupDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupDescriptionEntryFormat1(&mut self, ctx: &ReportGroupDescriptionEntryFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupDescriptionEntryFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupDescriptionEntryFormat2(&mut self, ctx: &ReportGroupDescriptionEntryFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupDescriptionEntryFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupDescriptionEntryFormat3(&mut self, ctx: &ReportGroupDescriptionEntryFormat3Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupDescriptionEntryFormat3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupBlankWhenZeroClause(&mut self, ctx: &ReportGroupBlankWhenZeroClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupBlankWhenZeroClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupColumnNumberClause(&mut self, ctx: &ReportGroupColumnNumberClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupColumnNumberClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupIndicateClause(&mut self, ctx: &ReportGroupIndicateClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupIndicateClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupJustifiedClause(&mut self, ctx: &ReportGroupJustifiedClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupJustifiedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupLineNumberClause(&mut self, ctx: &ReportGroupLineNumberClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupLineNumberClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupLineNumberNextPage(&mut self, ctx: &ReportGroupLineNumberNextPageContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupLineNumberNextPage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupLineNumberPlus(&mut self, ctx: &ReportGroupLineNumberPlusContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupLineNumberPlus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupNextGroupClause(&mut self, ctx: &ReportGroupNextGroupClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupNextGroupClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupNextGroupPlus(&mut self, ctx: &ReportGroupNextGroupPlusContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupNextGroupPlus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupNextGroupNextPage(&mut self, ctx: &ReportGroupNextGroupNextPageContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupNextGroupNextPage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupPictureClause(&mut self, ctx: &ReportGroupPictureClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupPictureClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupResetClause(&mut self, ctx: &ReportGroupResetClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupResetClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupSignClause(&mut self, ctx: &ReportGroupSignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupSignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupSourceClause(&mut self, ctx: &ReportGroupSourceClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupSourceClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupSumClause(&mut self, ctx: &ReportGroupSumClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupSumClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeClause(&mut self, ctx: &ReportGroupTypeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeReportHeading(&mut self, ctx: &ReportGroupTypeReportHeadingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeReportHeading(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypePageHeading(&mut self, ctx: &ReportGroupTypePageHeadingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypePageHeading(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeControlHeading(&mut self, ctx: &ReportGroupTypeControlHeadingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeControlHeading(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeDetail(&mut self, ctx: &ReportGroupTypeDetailContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeDetail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeControlFooting(&mut self, ctx: &ReportGroupTypeControlFootingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeControlFooting(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupUsageClause(&mut self, ctx: &ReportGroupUsageClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupUsageClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypePageFooting(&mut self, ctx: &ReportGroupTypePageFootingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypePageFooting(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupTypeReportFooting(&mut self, ctx: &ReportGroupTypeReportFootingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupTypeReportFooting(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportGroupValueClause(&mut self, ctx: &ReportGroupValueClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportGroupValueClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_programLibrarySection(&mut self, ctx: &ProgramLibrarySectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_programLibrarySection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryDescriptionEntry(&mut self, ctx: &LibraryDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryDescriptionEntryFormat1(&mut self, ctx: &LibraryDescriptionEntryFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryDescriptionEntryFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryDescriptionEntryFormat2(&mut self, ctx: &LibraryDescriptionEntryFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryDescriptionEntryFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryAttributeClauseFormat1(&mut self, ctx: &LibraryAttributeClauseFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryAttributeClauseFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryAttributeClauseFormat2(&mut self, ctx: &LibraryAttributeClauseFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryAttributeClauseFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryAttributeFunction(&mut self, ctx: &LibraryAttributeFunctionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryAttributeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryAttributeParameter(&mut self, ctx: &LibraryAttributeParameterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryAttributeParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryAttributeTitle(&mut self, ctx: &LibraryAttributeTitleContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryAttributeTitle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureClauseFormat1(&mut self, ctx: &LibraryEntryProcedureClauseFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureClauseFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureClauseFormat2(&mut self, ctx: &LibraryEntryProcedureClauseFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureClauseFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureForClause(&mut self, ctx: &LibraryEntryProcedureForClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureForClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureGivingClause(&mut self, ctx: &LibraryEntryProcedureGivingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureGivingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureUsingClause(&mut self, ctx: &LibraryEntryProcedureUsingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureUsingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureUsingName(&mut self, ctx: &LibraryEntryProcedureUsingNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureUsingName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureWithClause(&mut self, ctx: &LibraryEntryProcedureWithClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureWithClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryEntryProcedureWithName(&mut self, ctx: &LibraryEntryProcedureWithNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryEntryProcedureWithName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryIsCommonClause(&mut self, ctx: &LibraryIsCommonClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryIsCommonClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryIsGlobalClause(&mut self, ctx: &LibraryIsGlobalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryIsGlobalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescriptionEntry(&mut self, ctx: &DataDescriptionEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescriptionEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescriptionEntryFormat1(&mut self, ctx: &DataDescriptionEntryFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescriptionEntryFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescriptionEntryFormat2(&mut self, ctx: &DataDescriptionEntryFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescriptionEntryFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescriptionEntryFormat3(&mut self, ctx: &DataDescriptionEntryFormat3Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescriptionEntryFormat3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescriptionEntryExecSql(&mut self, ctx: &DataDescriptionEntryExecSqlContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescriptionEntryExecSql(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataAlignedClause(&mut self, ctx: &DataAlignedClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataAlignedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataBlankWhenZeroClause(&mut self, ctx: &DataBlankWhenZeroClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataBlankWhenZeroClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataCommonOwnLocalClause(&mut self, ctx: &DataCommonOwnLocalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataCommonOwnLocalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataExternalClause(&mut self, ctx: &DataExternalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataExternalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataGlobalClause(&mut self, ctx: &DataGlobalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataGlobalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataIntegerStringClause(&mut self, ctx: &DataIntegerStringClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataIntegerStringClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataJustifiedClause(&mut self, ctx: &DataJustifiedClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataJustifiedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataOccursClause(&mut self, ctx: &DataOccursClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataOccursClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataOccursTo(&mut self, ctx: &DataOccursToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataOccursTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataOccursSort(&mut self, ctx: &DataOccursSortContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataOccursSort(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataPictureClause(&mut self, ctx: &DataPictureClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataPictureClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pictureString(&mut self, ctx: &PictureStringContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_pictureString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pictureChars(&mut self, ctx: &PictureCharsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_pictureChars(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pictureCardinality(&mut self, ctx: &PictureCardinalityContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_pictureCardinality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataReceivedByClause(&mut self, ctx: &DataReceivedByClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataReceivedByClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataRecordAreaClause(&mut self, ctx: &DataRecordAreaClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataRecordAreaClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataRedefinesClause(&mut self, ctx: &DataRedefinesClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataRedefinesClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataRenamesClause(&mut self, ctx: &DataRenamesClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataRenamesClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataSignClause(&mut self, ctx: &DataSignClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataSignClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataSynchronizedClause(&mut self, ctx: &DataSynchronizedClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataSynchronizedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataThreadLocalClause(&mut self, ctx: &DataThreadLocalClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataThreadLocalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataTypeClause(&mut self, ctx: &DataTypeClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataTypeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataTypeDefClause(&mut self, ctx: &DataTypeDefClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataTypeDefClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataUsageClause(&mut self, ctx: &DataUsageClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataUsageClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataUsingClause(&mut self, ctx: &DataUsingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataUsingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataValueClause(&mut self, ctx: &DataValueClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataValueClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataValueInterval(&mut self, ctx: &DataValueIntervalContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataValueInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataValueIntervalFrom(&mut self, ctx: &DataValueIntervalFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataValueIntervalFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataValueIntervalTo(&mut self, ctx: &DataValueIntervalToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataValueIntervalTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataWithLowerBoundsClause(&mut self, ctx: &DataWithLowerBoundsClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataWithLowerBoundsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivision(&mut self, ctx: &ProcedureDivisionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivision(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionUsingClause(&mut self, ctx: &ProcedureDivisionUsingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionUsingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionGivingClause(&mut self, ctx: &ProcedureDivisionGivingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionGivingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionUsingParameter(&mut self, ctx: &ProcedureDivisionUsingParameterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionUsingParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionByReferencePhrase(&mut self, ctx: &ProcedureDivisionByReferencePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionByReferencePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionByReference(&mut self, ctx: &ProcedureDivisionByReferenceContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionByReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionByValuePhrase(&mut self, ctx: &ProcedureDivisionByValuePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionByValuePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionByValue(&mut self, ctx: &ProcedureDivisionByValueContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionByValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDeclaratives(&mut self, ctx: &ProcedureDeclarativesContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDeclaratives(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDeclarative(&mut self, ctx: &ProcedureDeclarativeContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDeclarative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureSectionHeader(&mut self, ctx: &ProcedureSectionHeaderContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureSectionHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureDivisionBody(&mut self, ctx: &ProcedureDivisionBodyContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureDivisionBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureSection(&mut self, ctx: &ProcedureSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paragraphs(&mut self, ctx: &ParagraphsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_paragraphs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paragraph(&mut self, ctx: &ParagraphContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_paragraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sentence(&mut self, ctx: &SentenceContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sentence(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_acceptStatement(&mut self, ctx: &AcceptStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_acceptStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_acceptFromDateStatement(&mut self, ctx: &AcceptFromDateStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_acceptFromDateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_acceptFromMnemonicStatement(&mut self, ctx: &AcceptFromMnemonicStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_acceptFromMnemonicStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_acceptFromEscapeKeyStatement(&mut self, ctx: &AcceptFromEscapeKeyStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_acceptFromEscapeKeyStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_acceptMessageCountStatement(&mut self, ctx: &AcceptMessageCountStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_acceptMessageCountStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addStatement(&mut self, ctx: &AddStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addToStatement(&mut self, ctx: &AddToStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addToStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addToGivingStatement(&mut self, ctx: &AddToGivingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addToGivingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addCorrespondingStatement(&mut self, ctx: &AddCorrespondingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addCorrespondingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addFrom(&mut self, ctx: &AddFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addTo(&mut self, ctx: &AddToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addToGiving(&mut self, ctx: &AddToGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addToGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addGiving(&mut self, ctx: &AddGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_addGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alteredGoTo(&mut self, ctx: &AlteredGoToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alteredGoTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterStatement(&mut self, ctx: &AlterStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterProceedTo(&mut self, ctx: &AlterProceedToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alterProceedTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callUsingPhrase(&mut self, ctx: &CallUsingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callUsingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callUsingParameter(&mut self, ctx: &CallUsingParameterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callUsingParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByReferencePhrase(&mut self, ctx: &CallByReferencePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByReferencePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByReference(&mut self, ctx: &CallByReferenceContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByValuePhrase(&mut self, ctx: &CallByValuePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByValuePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByValue(&mut self, ctx: &CallByValueContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByContentPhrase(&mut self, ctx: &CallByContentPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByContentPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callByContent(&mut self, ctx: &CallByContentContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callByContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callGivingPhrase(&mut self, ctx: &CallGivingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_callGivingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cancelStatement(&mut self, ctx: &CancelStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cancelStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cancelCall(&mut self, ctx: &CancelCallContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cancelCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closeStatement(&mut self, ctx: &CloseStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closeFile(&mut self, ctx: &CloseFileContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closeFile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closeReelUnitStatement(&mut self, ctx: &CloseReelUnitStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closeReelUnitStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closeRelativeStatement(&mut self, ctx: &CloseRelativeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closeRelativeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closePortFileIOStatement(&mut self, ctx: &ClosePortFileIOStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closePortFileIOStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closePortFileIOUsing(&mut self, ctx: &ClosePortFileIOUsingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closePortFileIOUsing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closePortFileIOUsingCloseDisposition(&mut self, ctx: &ClosePortFileIOUsingCloseDispositionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closePortFileIOUsingCloseDisposition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closePortFileIOUsingAssociatedData(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closePortFileIOUsingAssociatedData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closePortFileIOUsingAssociatedDataLength(&mut self, ctx: &ClosePortFileIOUsingAssociatedDataLengthContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_closePortFileIOUsingAssociatedDataLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computeStatement(&mut self, ctx: &ComputeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_computeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computeStore(&mut self, ctx: &ComputeStoreContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_computeStore(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_continueStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_deleteStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_disableStatement(&mut self, ctx: &DisableStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_disableStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_displayStatement(&mut self, ctx: &DisplayStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_displayStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_displayOperand(&mut self, ctx: &DisplayOperandContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_displayOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_displayAt(&mut self, ctx: &DisplayAtContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_displayAt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_displayUpon(&mut self, ctx: &DisplayUponContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_displayUpon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_displayWith(&mut self, ctx: &DisplayWithContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_displayWith(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideStatement(&mut self, ctx: &DivideStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideIntoStatement(&mut self, ctx: &DivideIntoStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideIntoStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideIntoGivingStatement(&mut self, ctx: &DivideIntoGivingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideIntoGivingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideByGivingStatement(&mut self, ctx: &DivideByGivingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideByGivingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideGivingPhrase(&mut self, ctx: &DivideGivingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideGivingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideInto(&mut self, ctx: &DivideIntoContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideInto(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideGiving(&mut self, ctx: &DivideGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_divideRemainder(&mut self, ctx: &DivideRemainderContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_divideRemainder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enableStatement(&mut self, ctx: &EnableStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_enableStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_entryStatement(&mut self, ctx: &EntryStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_entryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateSelect(&mut self, ctx: &EvaluateSelectContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateSelect(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateAlsoSelect(&mut self, ctx: &EvaluateAlsoSelectContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateAlsoSelect(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateWhenPhrase(&mut self, ctx: &EvaluateWhenPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateWhenPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateWhen(&mut self, ctx: &EvaluateWhenContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateWhen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateCondition(&mut self, ctx: &EvaluateConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateThrough(&mut self, ctx: &EvaluateThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateAlsoCondition(&mut self, ctx: &EvaluateAlsoConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateAlsoCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateWhenOther(&mut self, ctx: &EvaluateWhenOtherContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateWhenOther(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateValue(&mut self, ctx: &EvaluateValueContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_evaluateValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execCicsStatement(&mut self, ctx: &ExecCicsStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_execCicsStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execSqlStatement(&mut self, ctx: &ExecSqlStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_execSqlStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_execSqlImsStatement(&mut self, ctx: &ExecSqlImsStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_execSqlImsStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exhibitStatement(&mut self, ctx: &ExhibitStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_exhibitStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exhibitOperand(&mut self, ctx: &ExhibitOperandContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_exhibitOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exitStatement(&mut self, ctx: &ExitStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_exitStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_generateStatement(&mut self, ctx: &GenerateStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_generateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gobackStatement(&mut self, ctx: &GobackStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_gobackStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_goToStatement(&mut self, ctx: &GoToStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_goToStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_goToStatementSimple(&mut self, ctx: &GoToStatementSimpleContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_goToStatementSimple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_goToDependingOnStatement(&mut self, ctx: &GoToDependingOnStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_goToDependingOnStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_ifStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifThen(&mut self, ctx: &IfThenContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_ifThen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifElse(&mut self, ctx: &IfElseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_ifElse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializeStatement(&mut self, ctx: &InitializeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_initializeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializeReplacingPhrase(&mut self, ctx: &InitializeReplacingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_initializeReplacingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializeReplacingBy(&mut self, ctx: &InitializeReplacingByContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_initializeReplacingBy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initiateStatement(&mut self, ctx: &InitiateStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_initiateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectStatement(&mut self, ctx: &InspectStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectTallyingPhrase(&mut self, ctx: &InspectTallyingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectTallyingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectReplacingPhrase(&mut self, ctx: &InspectReplacingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectReplacingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectTallyingReplacingPhrase(&mut self, ctx: &InspectTallyingReplacingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectTallyingReplacingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectConvertingPhrase(&mut self, ctx: &InspectConvertingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectConvertingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectFor(&mut self, ctx: &InspectForContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectFor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectCharacters(&mut self, ctx: &InspectCharactersContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectCharacters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectReplacingCharacters(&mut self, ctx: &InspectReplacingCharactersContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectReplacingCharacters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectAllLeadings(&mut self, ctx: &InspectAllLeadingsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectAllLeadings(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectReplacingAllLeadings(&mut self, ctx: &InspectReplacingAllLeadingsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectReplacingAllLeadings(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectAllLeading(&mut self, ctx: &InspectAllLeadingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectAllLeading(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectReplacingAllLeading(&mut self, ctx: &InspectReplacingAllLeadingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectReplacingAllLeading(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectBy(&mut self, ctx: &InspectByContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectBy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectTo(&mut self, ctx: &InspectToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inspectBeforeAfter(&mut self, ctx: &InspectBeforeAfterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inspectBeforeAfter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeStatement(&mut self, ctx: &MergeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeOnKeyClause(&mut self, ctx: &MergeOnKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeOnKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeCollatingSequencePhrase(&mut self, ctx: &MergeCollatingSequencePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeCollatingSequencePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeCollatingAlphanumeric(&mut self, ctx: &MergeCollatingAlphanumericContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeCollatingAlphanumeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeCollatingNational(&mut self, ctx: &MergeCollatingNationalContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeCollatingNational(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeUsing(&mut self, ctx: &MergeUsingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeUsing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeOutputProcedurePhrase(&mut self, ctx: &MergeOutputProcedurePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeOutputProcedurePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeOutputThrough(&mut self, ctx: &MergeOutputThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeOutputThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeGivingPhrase(&mut self, ctx: &MergeGivingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeGivingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeGiving(&mut self, ctx: &MergeGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mergeGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moveStatement(&mut self, ctx: &MoveStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_moveStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moveToStatement(&mut self, ctx: &MoveToStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_moveToStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moveToSendingArea(&mut self, ctx: &MoveToSendingAreaContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_moveToSendingArea(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moveCorrespondingToStatement(&mut self, ctx: &MoveCorrespondingToStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_moveCorrespondingToStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moveCorrespondingToSendingArea(&mut self, ctx: &MoveCorrespondingToSendingAreaContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_moveCorrespondingToSendingArea(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyStatement(&mut self, ctx: &MultiplyStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyRegular(&mut self, ctx: &MultiplyRegularContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyRegular(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyRegularOperand(&mut self, ctx: &MultiplyRegularOperandContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyRegularOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyGiving(&mut self, ctx: &MultiplyGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyGivingOperand(&mut self, ctx: &MultiplyGivingOperandContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyGivingOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyGivingResult(&mut self, ctx: &MultiplyGivingResultContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multiplyGivingResult(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openStatement(&mut self, ctx: &OpenStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openInputStatement(&mut self, ctx: &OpenInputStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openInputStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openInput(&mut self, ctx: &OpenInputContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openInput(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openOutputStatement(&mut self, ctx: &OpenOutputStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openOutputStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openOutput(&mut self, ctx: &OpenOutputContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openOutput(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openIOStatement(&mut self, ctx: &OpenIOStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openIOStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_openExtendStatement(&mut self, ctx: &OpenExtendStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_openExtendStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performStatement(&mut self, ctx: &PerformStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performInlineStatement(&mut self, ctx: &PerformInlineStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performInlineStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performProcedureStatement(&mut self, ctx: &PerformProcedureStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performProcedureStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performType(&mut self, ctx: &PerformTypeContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performTimes(&mut self, ctx: &PerformTimesContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performTimes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performUntil(&mut self, ctx: &PerformUntilContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performUntil(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performVarying(&mut self, ctx: &PerformVaryingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performVarying(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performVaryingClause(&mut self, ctx: &PerformVaryingClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performVaryingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performVaryingPhrase(&mut self, ctx: &PerformVaryingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performVaryingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performAfter(&mut self, ctx: &PerformAfterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performAfter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performFrom(&mut self, ctx: &PerformFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performBy(&mut self, ctx: &PerformByContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performBy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_performTestClause(&mut self, ctx: &PerformTestClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_performTestClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_purgeStatement(&mut self, ctx: &PurgeStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_purgeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readStatement(&mut self, ctx: &ReadStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_readStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readInto(&mut self, ctx: &ReadIntoContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_readInto(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readWith(&mut self, ctx: &ReadWithContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_readWith(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readKey(&mut self, ctx: &ReadKeyContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_readKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveStatement(&mut self, ctx: &ReceiveStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveFromStatement(&mut self, ctx: &ReceiveFromStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveFromStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveFrom(&mut self, ctx: &ReceiveFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveIntoStatement(&mut self, ctx: &ReceiveIntoStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveIntoStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveNoData(&mut self, ctx: &ReceiveNoDataContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveNoData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveWithData(&mut self, ctx: &ReceiveWithDataContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveWithData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveBefore(&mut self, ctx: &ReceiveBeforeContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveBefore(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveWith(&mut self, ctx: &ReceiveWithContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveWith(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveThread(&mut self, ctx: &ReceiveThreadContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveThread(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveSize(&mut self, ctx: &ReceiveSizeContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveSize(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveStatus(&mut self, ctx: &ReceiveStatusContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_receiveStatus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_releaseStatement(&mut self, ctx: &ReleaseStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_releaseStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_returnStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnInto(&mut self, ctx: &ReturnIntoContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_returnInto(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rewriteStatement(&mut self, ctx: &RewriteStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rewriteStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rewriteFrom(&mut self, ctx: &RewriteFromContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_rewriteFrom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_searchStatement(&mut self, ctx: &SearchStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_searchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_searchVarying(&mut self, ctx: &SearchVaryingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_searchVarying(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_searchWhen(&mut self, ctx: &SearchWhenContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_searchWhen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendStatement(&mut self, ctx: &SendStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendStatementSync(&mut self, ctx: &SendStatementSyncContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendStatementSync(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendStatementAsync(&mut self, ctx: &SendStatementAsyncContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendStatementAsync(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendFromPhrase(&mut self, ctx: &SendFromPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendFromPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendWithPhrase(&mut self, ctx: &SendWithPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendWithPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendReplacingPhrase(&mut self, ctx: &SendReplacingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendReplacingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendAdvancingPhrase(&mut self, ctx: &SendAdvancingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendAdvancingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendAdvancingPage(&mut self, ctx: &SendAdvancingPageContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendAdvancingPage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendAdvancingLines(&mut self, ctx: &SendAdvancingLinesContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendAdvancingLines(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sendAdvancingMnemonic(&mut self, ctx: &SendAdvancingMnemonicContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sendAdvancingMnemonic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setToStatement(&mut self, ctx: &SetToStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setToStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setUpDownByStatement(&mut self, ctx: &SetUpDownByStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setUpDownByStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setTo(&mut self, ctx: &SetToContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setTo(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setToValue(&mut self, ctx: &SetToValueContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setToValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setByValue(&mut self, ctx: &SetByValueContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_setByValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortStatement(&mut self, ctx: &SortStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortOnKeyClause(&mut self, ctx: &SortOnKeyClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortOnKeyClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortDuplicatesPhrase(&mut self, ctx: &SortDuplicatesPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortDuplicatesPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortCollatingSequencePhrase(&mut self, ctx: &SortCollatingSequencePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortCollatingSequencePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortCollatingAlphanumeric(&mut self, ctx: &SortCollatingAlphanumericContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortCollatingAlphanumeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortCollatingNational(&mut self, ctx: &SortCollatingNationalContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortCollatingNational(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortInputProcedurePhrase(&mut self, ctx: &SortInputProcedurePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortInputProcedurePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortInputThrough(&mut self, ctx: &SortInputThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortInputThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortUsing(&mut self, ctx: &SortUsingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortUsing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortOutputProcedurePhrase(&mut self, ctx: &SortOutputProcedurePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortOutputProcedurePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortOutputThrough(&mut self, ctx: &SortOutputThroughContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortOutputThrough(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortGivingPhrase(&mut self, ctx: &SortGivingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortGivingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortGiving(&mut self, ctx: &SortGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sortGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_startStatement(&mut self, ctx: &StartStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_startStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_startKey(&mut self, ctx: &StartKeyContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_startKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stopStatement(&mut self, ctx: &StopStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stopStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringStatement(&mut self, ctx: &StringStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringSendingPhrase(&mut self, ctx: &StringSendingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringSendingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringSending(&mut self, ctx: &StringSendingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringSending(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringDelimitedByPhrase(&mut self, ctx: &StringDelimitedByPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringDelimitedByPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringForPhrase(&mut self, ctx: &StringForPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringForPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringIntoPhrase(&mut self, ctx: &StringIntoPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringIntoPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringWithPointerPhrase(&mut self, ctx: &StringWithPointerPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_stringWithPointerPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractStatement(&mut self, ctx: &SubtractStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractFromStatement(&mut self, ctx: &SubtractFromStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractFromStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractFromGivingStatement(&mut self, ctx: &SubtractFromGivingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractFromGivingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractCorrespondingStatement(&mut self, ctx: &SubtractCorrespondingStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractCorrespondingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractSubtrahend(&mut self, ctx: &SubtractSubtrahendContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractSubtrahend(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractMinuend(&mut self, ctx: &SubtractMinuendContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractMinuend(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractMinuendGiving(&mut self, ctx: &SubtractMinuendGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractMinuendGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractGiving(&mut self, ctx: &SubtractGivingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractGiving(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtractMinuendCorresponding(&mut self, ctx: &SubtractMinuendCorrespondingContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subtractMinuendCorresponding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_terminateStatement(&mut self, ctx: &TerminateStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_terminateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringStatement(&mut self, ctx: &UnstringStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringSendingPhrase(&mut self, ctx: &UnstringSendingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringSendingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringDelimitedByPhrase(&mut self, ctx: &UnstringDelimitedByPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringDelimitedByPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringOrAllPhrase(&mut self, ctx: &UnstringOrAllPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringOrAllPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringIntoPhrase(&mut self, ctx: &UnstringIntoPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringIntoPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringInto(&mut self, ctx: &UnstringIntoContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringInto(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringDelimiterIn(&mut self, ctx: &UnstringDelimiterInContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringDelimiterIn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringCountIn(&mut self, ctx: &UnstringCountInContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringCountIn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringWithPointerPhrase(&mut self, ctx: &UnstringWithPointerPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringWithPointerPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unstringTallyingPhrase(&mut self, ctx: &UnstringTallyingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_unstringTallyingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useStatement(&mut self, ctx: &UseStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_useStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useAfterClause(&mut self, ctx: &UseAfterClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_useAfterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useAfterOn(&mut self, ctx: &UseAfterOnContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_useAfterOn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useDebugClause(&mut self, ctx: &UseDebugClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_useDebugClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useDebugOn(&mut self, ctx: &UseDebugOnContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_useDebugOn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeStatement(&mut self, ctx: &WriteStatementContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeFromPhrase(&mut self, ctx: &WriteFromPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeFromPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeAdvancingPhrase(&mut self, ctx: &WriteAdvancingPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeAdvancingPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeAdvancingPage(&mut self, ctx: &WriteAdvancingPageContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeAdvancingPage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeAdvancingLines(&mut self, ctx: &WriteAdvancingLinesContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeAdvancingLines(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeAdvancingMnemonic(&mut self, ctx: &WriteAdvancingMnemonicContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeAdvancingMnemonic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeAtEndOfPagePhrase(&mut self, ctx: &WriteAtEndOfPagePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeAtEndOfPagePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_writeNotAtEndOfPagePhrase(&mut self, ctx: &WriteNotAtEndOfPagePhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_writeNotAtEndOfPagePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atEndPhrase(&mut self, ctx: &AtEndPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_atEndPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notAtEndPhrase(&mut self, ctx: &NotAtEndPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_notAtEndPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_invalidKeyPhrase(&mut self, ctx: &InvalidKeyPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_invalidKeyPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notInvalidKeyPhrase(&mut self, ctx: &NotInvalidKeyPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_notInvalidKeyPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onOverflowPhrase(&mut self, ctx: &OnOverflowPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_onOverflowPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notOnOverflowPhrase(&mut self, ctx: &NotOnOverflowPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_notOnOverflowPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onSizeErrorPhrase(&mut self, ctx: &OnSizeErrorPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_onSizeErrorPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notOnSizeErrorPhrase(&mut self, ctx: &NotOnSizeErrorPhraseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_notOnSizeErrorPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onExceptionClause(&mut self, ctx: &OnExceptionClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_onExceptionClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notOnExceptionClause(&mut self, ctx: &NotOnExceptionClauseContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_notOnExceptionClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticExpression(&mut self, ctx: &ArithmeticExpressionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_arithmeticExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_plusMinus(&mut self, ctx: &PlusMinusContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_plusMinus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multDivs(&mut self, ctx: &MultDivsContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multDivs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multDiv(&mut self, ctx: &MultDivContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_multDiv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powers(&mut self, ctx: &PowersContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_powers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_power(&mut self, ctx: &PowerContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_power(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_basis(&mut self, ctx: &BasisContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_basis(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_condition(&mut self, ctx: &ConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_condition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_andOrCondition(&mut self, ctx: &AndOrConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_andOrCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_combinableCondition(&mut self, ctx: &CombinableConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_combinableCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleCondition(&mut self, ctx: &SimpleConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_simpleCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classCondition(&mut self, ctx: &ClassConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_classCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionNameReference(&mut self, ctx: &ConditionNameReferenceContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_conditionNameReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionNameSubscriptReference(&mut self, ctx: &ConditionNameSubscriptReferenceContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_conditionNameSubscriptReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationCondition(&mut self, ctx: &RelationConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationSignCondition(&mut self, ctx: &RelationSignConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationSignCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationArithmeticComparison(&mut self, ctx: &RelationArithmeticComparisonContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationArithmeticComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationCombinedComparison(&mut self, ctx: &RelationCombinedComparisonContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationCombinedComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationCombinedCondition(&mut self, ctx: &RelationCombinedConditionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationCombinedCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalOperator(&mut self, ctx: &RelationalOperatorContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_relationalOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_abbreviation(&mut self, ctx: &AbbreviationContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_abbreviation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableCall(&mut self, ctx: &TableCallContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_tableCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_referenceModifier(&mut self, ctx: &ReferenceModifierContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_referenceModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_characterPosition(&mut self, ctx: &CharacterPositionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_characterPosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_length(&mut self, ctx: &LengthContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_length(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_subscript_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_argument(&mut self, ctx: &ArgumentContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_argument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedDataName(&mut self, ctx: &QualifiedDataNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedDataName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedDataNameFormat1(&mut self, ctx: &QualifiedDataNameFormat1Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedDataNameFormat1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedDataNameFormat2(&mut self, ctx: &QualifiedDataNameFormat2Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedDataNameFormat2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedDataNameFormat3(&mut self, ctx: &QualifiedDataNameFormat3Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedDataNameFormat3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedDataNameFormat4(&mut self, ctx: &QualifiedDataNameFormat4Context<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedDataNameFormat4(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedInData(&mut self, ctx: &QualifiedInDataContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_qualifiedInData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inData(&mut self, ctx: &InDataContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inFile(&mut self, ctx: &InFileContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inFile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inMnemonic(&mut self, ctx: &InMnemonicContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inMnemonic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inSection(&mut self, ctx: &InSectionContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inLibrary(&mut self, ctx: &InLibraryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inLibrary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inTable(&mut self, ctx: &InTableContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_inTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alphabetName(&mut self, ctx: &AlphabetNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_alphabetName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentName(&mut self, ctx: &AssignmentNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_assignmentName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_basisName(&mut self, ctx: &BasisNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_basisName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cdName(&mut self, ctx: &CdNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cdName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_className(&mut self, ctx: &ClassNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_className(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computerName(&mut self, ctx: &ComputerNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_computerName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionName(&mut self, ctx: &ConditionNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_conditionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataName(&mut self, ctx: &DataNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dataDescName(&mut self, ctx: &DataDescNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_dataDescName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_environmentName(&mut self, ctx: &EnvironmentNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_environmentName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fileName(&mut self, ctx: &FileNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_fileName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_functionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexName(&mut self, ctx: &IndexNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_indexName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_languageName(&mut self, ctx: &LanguageNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_languageName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryName(&mut self, ctx: &LibraryNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_libraryName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localName(&mut self, ctx: &LocalNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_localName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonicName(&mut self, ctx: &MnemonicNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_mnemonicName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paragraphName(&mut self, ctx: &ParagraphNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_paragraphName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_procedureName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_programName(&mut self, ctx: &ProgramNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_programName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordName(&mut self, ctx: &RecordNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_recordName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reportName(&mut self, ctx: &ReportNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_reportName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routineName(&mut self, ctx: &RoutineNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_routineName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_screenName(&mut self, ctx: &ScreenNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_screenName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sectionName(&mut self, ctx: &SectionNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_sectionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_systemName(&mut self, ctx: &SystemNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_systemName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symbolicCharacter(&mut self, ctx: &SymbolicCharacterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_symbolicCharacter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_textName(&mut self, ctx: &TextNameContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_textName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cobolWord(&mut self, ctx: &CobolWordContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cobolWord(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_numericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_integerLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cicsDfhRespLiteral(&mut self, ctx: &CicsDfhRespLiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cicsDfhRespLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cicsDfhValueLiteral(&mut self, ctx: &CicsDfhValueLiteralContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_cicsDfhValueLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_figurativeConstant(&mut self, ctx: &FigurativeConstantContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_figurativeConstant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_specialRegister(&mut self, ctx: &SpecialRegisterContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_specialRegister(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commentEntry(&mut self, ctx: &CommentEntryContext<'input>){
		let result = <Self as Cobol85VisitorCompat>::visit_commentEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}