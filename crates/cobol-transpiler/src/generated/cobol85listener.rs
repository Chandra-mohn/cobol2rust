#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/cobol2rust/grammar/Cobol85.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::cobol85parser::*;

pub trait Cobol85Listener<'input> : ParseTreeListener<'input,Cobol85ParserContextType>{
/**
 * Enter a parse tree produced by {@link Cobol85Parser#startRule}.
 * @param ctx the parse tree
 */
fn enter_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#startRule}.
 * @param ctx the parse tree
 */
fn exit_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#programUnit}.
 * @param ctx the parse tree
 */
fn enter_programUnit(&mut self, _ctx: &ProgramUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#programUnit}.
 * @param ctx the parse tree
 */
fn exit_programUnit(&mut self, _ctx: &ProgramUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#endProgramStatement}.
 * @param ctx the parse tree
 */
fn enter_endProgramStatement(&mut self, _ctx: &EndProgramStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#endProgramStatement}.
 * @param ctx the parse tree
 */
fn exit_endProgramStatement(&mut self, _ctx: &EndProgramStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#identificationDivision}.
 * @param ctx the parse tree
 */
fn enter_identificationDivision(&mut self, _ctx: &IdentificationDivisionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#identificationDivision}.
 * @param ctx the parse tree
 */
fn exit_identificationDivision(&mut self, _ctx: &IdentificationDivisionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#identificationDivisionBody}.
 * @param ctx the parse tree
 */
fn enter_identificationDivisionBody(&mut self, _ctx: &IdentificationDivisionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#identificationDivisionBody}.
 * @param ctx the parse tree
 */
fn exit_identificationDivisionBody(&mut self, _ctx: &IdentificationDivisionBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#programIdParagraph}.
 * @param ctx the parse tree
 */
fn enter_programIdParagraph(&mut self, _ctx: &ProgramIdParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#programIdParagraph}.
 * @param ctx the parse tree
 */
fn exit_programIdParagraph(&mut self, _ctx: &ProgramIdParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#authorParagraph}.
 * @param ctx the parse tree
 */
fn enter_authorParagraph(&mut self, _ctx: &AuthorParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#authorParagraph}.
 * @param ctx the parse tree
 */
fn exit_authorParagraph(&mut self, _ctx: &AuthorParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#installationParagraph}.
 * @param ctx the parse tree
 */
fn enter_installationParagraph(&mut self, _ctx: &InstallationParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#installationParagraph}.
 * @param ctx the parse tree
 */
fn exit_installationParagraph(&mut self, _ctx: &InstallationParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dateWrittenParagraph}.
 * @param ctx the parse tree
 */
fn enter_dateWrittenParagraph(&mut self, _ctx: &DateWrittenParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dateWrittenParagraph}.
 * @param ctx the parse tree
 */
fn exit_dateWrittenParagraph(&mut self, _ctx: &DateWrittenParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dateCompiledParagraph}.
 * @param ctx the parse tree
 */
fn enter_dateCompiledParagraph(&mut self, _ctx: &DateCompiledParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dateCompiledParagraph}.
 * @param ctx the parse tree
 */
fn exit_dateCompiledParagraph(&mut self, _ctx: &DateCompiledParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#securityParagraph}.
 * @param ctx the parse tree
 */
fn enter_securityParagraph(&mut self, _ctx: &SecurityParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#securityParagraph}.
 * @param ctx the parse tree
 */
fn exit_securityParagraph(&mut self, _ctx: &SecurityParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#remarksParagraph}.
 * @param ctx the parse tree
 */
fn enter_remarksParagraph(&mut self, _ctx: &RemarksParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#remarksParagraph}.
 * @param ctx the parse tree
 */
fn exit_remarksParagraph(&mut self, _ctx: &RemarksParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#environmentDivision}.
 * @param ctx the parse tree
 */
fn enter_environmentDivision(&mut self, _ctx: &EnvironmentDivisionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#environmentDivision}.
 * @param ctx the parse tree
 */
fn exit_environmentDivision(&mut self, _ctx: &EnvironmentDivisionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#environmentDivisionBody}.
 * @param ctx the parse tree
 */
fn enter_environmentDivisionBody(&mut self, _ctx: &EnvironmentDivisionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#environmentDivisionBody}.
 * @param ctx the parse tree
 */
fn exit_environmentDivisionBody(&mut self, _ctx: &EnvironmentDivisionBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#configurationSection}.
 * @param ctx the parse tree
 */
fn enter_configurationSection(&mut self, _ctx: &ConfigurationSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#configurationSection}.
 * @param ctx the parse tree
 */
fn exit_configurationSection(&mut self, _ctx: &ConfigurationSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#configurationSectionParagraph}.
 * @param ctx the parse tree
 */
fn enter_configurationSectionParagraph(&mut self, _ctx: &ConfigurationSectionParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#configurationSectionParagraph}.
 * @param ctx the parse tree
 */
fn exit_configurationSectionParagraph(&mut self, _ctx: &ConfigurationSectionParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sourceComputerParagraph}.
 * @param ctx the parse tree
 */
fn enter_sourceComputerParagraph(&mut self, _ctx: &SourceComputerParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sourceComputerParagraph}.
 * @param ctx the parse tree
 */
fn exit_sourceComputerParagraph(&mut self, _ctx: &SourceComputerParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#objectComputerParagraph}.
 * @param ctx the parse tree
 */
fn enter_objectComputerParagraph(&mut self, _ctx: &ObjectComputerParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#objectComputerParagraph}.
 * @param ctx the parse tree
 */
fn exit_objectComputerParagraph(&mut self, _ctx: &ObjectComputerParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#objectComputerClause}.
 * @param ctx the parse tree
 */
fn enter_objectComputerClause(&mut self, _ctx: &ObjectComputerClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#objectComputerClause}.
 * @param ctx the parse tree
 */
fn exit_objectComputerClause(&mut self, _ctx: &ObjectComputerClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#memorySizeClause}.
 * @param ctx the parse tree
 */
fn enter_memorySizeClause(&mut self, _ctx: &MemorySizeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#memorySizeClause}.
 * @param ctx the parse tree
 */
fn exit_memorySizeClause(&mut self, _ctx: &MemorySizeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#diskSizeClause}.
 * @param ctx the parse tree
 */
fn enter_diskSizeClause(&mut self, _ctx: &DiskSizeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#diskSizeClause}.
 * @param ctx the parse tree
 */
fn exit_diskSizeClause(&mut self, _ctx: &DiskSizeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#collatingSequenceClause}.
 * @param ctx the parse tree
 */
fn enter_collatingSequenceClause(&mut self, _ctx: &CollatingSequenceClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#collatingSequenceClause}.
 * @param ctx the parse tree
 */
fn exit_collatingSequenceClause(&mut self, _ctx: &CollatingSequenceClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseAlphanumeric}.
 * @param ctx the parse tree
 */
fn enter_collatingSequenceClauseAlphanumeric(&mut self, _ctx: &CollatingSequenceClauseAlphanumericContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseAlphanumeric}.
 * @param ctx the parse tree
 */
fn exit_collatingSequenceClauseAlphanumeric(&mut self, _ctx: &CollatingSequenceClauseAlphanumericContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseNational}.
 * @param ctx the parse tree
 */
fn enter_collatingSequenceClauseNational(&mut self, _ctx: &CollatingSequenceClauseNationalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#collatingSequenceClauseNational}.
 * @param ctx the parse tree
 */
fn exit_collatingSequenceClauseNational(&mut self, _ctx: &CollatingSequenceClauseNationalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#segmentLimitClause}.
 * @param ctx the parse tree
 */
fn enter_segmentLimitClause(&mut self, _ctx: &SegmentLimitClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#segmentLimitClause}.
 * @param ctx the parse tree
 */
fn exit_segmentLimitClause(&mut self, _ctx: &SegmentLimitClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#characterSetClause}.
 * @param ctx the parse tree
 */
fn enter_characterSetClause(&mut self, _ctx: &CharacterSetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#characterSetClause}.
 * @param ctx the parse tree
 */
fn exit_characterSetClause(&mut self, _ctx: &CharacterSetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#specialNamesParagraph}.
 * @param ctx the parse tree
 */
fn enter_specialNamesParagraph(&mut self, _ctx: &SpecialNamesParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#specialNamesParagraph}.
 * @param ctx the parse tree
 */
fn exit_specialNamesParagraph(&mut self, _ctx: &SpecialNamesParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#specialNameClause}.
 * @param ctx the parse tree
 */
fn enter_specialNameClause(&mut self, _ctx: &SpecialNameClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#specialNameClause}.
 * @param ctx the parse tree
 */
fn exit_specialNameClause(&mut self, _ctx: &SpecialNameClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetClause}.
 * @param ctx the parse tree
 */
fn enter_alphabetClause(&mut self, _ctx: &AlphabetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetClause}.
 * @param ctx the parse tree
 */
fn exit_alphabetClause(&mut self, _ctx: &AlphabetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat1}.
 * @param ctx the parse tree
 */
fn enter_alphabetClauseFormat1(&mut self, _ctx: &AlphabetClauseFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat1}.
 * @param ctx the parse tree
 */
fn exit_alphabetClauseFormat1(&mut self, _ctx: &AlphabetClauseFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetLiterals}.
 * @param ctx the parse tree
 */
fn enter_alphabetLiterals(&mut self, _ctx: &AlphabetLiteralsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetLiterals}.
 * @param ctx the parse tree
 */
fn exit_alphabetLiterals(&mut self, _ctx: &AlphabetLiteralsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetThrough}.
 * @param ctx the parse tree
 */
fn enter_alphabetThrough(&mut self, _ctx: &AlphabetThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetThrough}.
 * @param ctx the parse tree
 */
fn exit_alphabetThrough(&mut self, _ctx: &AlphabetThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetAlso}.
 * @param ctx the parse tree
 */
fn enter_alphabetAlso(&mut self, _ctx: &AlphabetAlsoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetAlso}.
 * @param ctx the parse tree
 */
fn exit_alphabetAlso(&mut self, _ctx: &AlphabetAlsoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat2}.
 * @param ctx the parse tree
 */
fn enter_alphabetClauseFormat2(&mut self, _ctx: &AlphabetClauseFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetClauseFormat2}.
 * @param ctx the parse tree
 */
fn exit_alphabetClauseFormat2(&mut self, _ctx: &AlphabetClauseFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#channelClause}.
 * @param ctx the parse tree
 */
fn enter_channelClause(&mut self, _ctx: &ChannelClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#channelClause}.
 * @param ctx the parse tree
 */
fn exit_channelClause(&mut self, _ctx: &ChannelClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#classClause}.
 * @param ctx the parse tree
 */
fn enter_classClause(&mut self, _ctx: &ClassClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#classClause}.
 * @param ctx the parse tree
 */
fn exit_classClause(&mut self, _ctx: &ClassClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#classClauseThrough}.
 * @param ctx the parse tree
 */
fn enter_classClauseThrough(&mut self, _ctx: &ClassClauseThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#classClauseThrough}.
 * @param ctx the parse tree
 */
fn exit_classClauseThrough(&mut self, _ctx: &ClassClauseThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#classClauseFrom}.
 * @param ctx the parse tree
 */
fn enter_classClauseFrom(&mut self, _ctx: &ClassClauseFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#classClauseFrom}.
 * @param ctx the parse tree
 */
fn exit_classClauseFrom(&mut self, _ctx: &ClassClauseFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#classClauseTo}.
 * @param ctx the parse tree
 */
fn enter_classClauseTo(&mut self, _ctx: &ClassClauseToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#classClauseTo}.
 * @param ctx the parse tree
 */
fn exit_classClauseTo(&mut self, _ctx: &ClassClauseToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#currencySignClause}.
 * @param ctx the parse tree
 */
fn enter_currencySignClause(&mut self, _ctx: &CurrencySignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#currencySignClause}.
 * @param ctx the parse tree
 */
fn exit_currencySignClause(&mut self, _ctx: &CurrencySignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#decimalPointClause}.
 * @param ctx the parse tree
 */
fn enter_decimalPointClause(&mut self, _ctx: &DecimalPointClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#decimalPointClause}.
 * @param ctx the parse tree
 */
fn exit_decimalPointClause(&mut self, _ctx: &DecimalPointClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#defaultComputationalSignClause}.
 * @param ctx the parse tree
 */
fn enter_defaultComputationalSignClause(&mut self, _ctx: &DefaultComputationalSignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#defaultComputationalSignClause}.
 * @param ctx the parse tree
 */
fn exit_defaultComputationalSignClause(&mut self, _ctx: &DefaultComputationalSignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#defaultDisplaySignClause}.
 * @param ctx the parse tree
 */
fn enter_defaultDisplaySignClause(&mut self, _ctx: &DefaultDisplaySignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#defaultDisplaySignClause}.
 * @param ctx the parse tree
 */
fn exit_defaultDisplaySignClause(&mut self, _ctx: &DefaultDisplaySignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#environmentSwitchNameClause}.
 * @param ctx the parse tree
 */
fn enter_environmentSwitchNameClause(&mut self, _ctx: &EnvironmentSwitchNameClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameClause}.
 * @param ctx the parse tree
 */
fn exit_environmentSwitchNameClause(&mut self, _ctx: &EnvironmentSwitchNameClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#environmentSwitchNameSpecialNamesStatusPhrase}.
 * @param ctx the parse tree
 */
fn enter_environmentSwitchNameSpecialNamesStatusPhrase(&mut self, _ctx: &EnvironmentSwitchNameSpecialNamesStatusPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#environmentSwitchNameSpecialNamesStatusPhrase}.
 * @param ctx the parse tree
 */
fn exit_environmentSwitchNameSpecialNamesStatusPhrase(&mut self, _ctx: &EnvironmentSwitchNameSpecialNamesStatusPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#odtClause}.
 * @param ctx the parse tree
 */
fn enter_odtClause(&mut self, _ctx: &OdtClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#odtClause}.
 * @param ctx the parse tree
 */
fn exit_odtClause(&mut self, _ctx: &OdtClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reserveNetworkClause}.
 * @param ctx the parse tree
 */
fn enter_reserveNetworkClause(&mut self, _ctx: &ReserveNetworkClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reserveNetworkClause}.
 * @param ctx the parse tree
 */
fn exit_reserveNetworkClause(&mut self, _ctx: &ReserveNetworkClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicCharactersClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicCharactersClause(&mut self, _ctx: &SymbolicCharactersClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicCharactersClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicCharactersClause(&mut self, _ctx: &SymbolicCharactersClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicCharacters}.
 * @param ctx the parse tree
 */
fn enter_symbolicCharacters(&mut self, _ctx: &SymbolicCharactersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicCharacters}.
 * @param ctx the parse tree
 */
fn exit_symbolicCharacters(&mut self, _ctx: &SymbolicCharactersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inputOutputSection}.
 * @param ctx the parse tree
 */
fn enter_inputOutputSection(&mut self, _ctx: &InputOutputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inputOutputSection}.
 * @param ctx the parse tree
 */
fn exit_inputOutputSection(&mut self, _ctx: &InputOutputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inputOutputSectionParagraph}.
 * @param ctx the parse tree
 */
fn enter_inputOutputSectionParagraph(&mut self, _ctx: &InputOutputSectionParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inputOutputSectionParagraph}.
 * @param ctx the parse tree
 */
fn exit_inputOutputSectionParagraph(&mut self, _ctx: &InputOutputSectionParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileControlParagraph}.
 * @param ctx the parse tree
 */
fn enter_fileControlParagraph(&mut self, _ctx: &FileControlParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileControlParagraph}.
 * @param ctx the parse tree
 */
fn exit_fileControlParagraph(&mut self, _ctx: &FileControlParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileControlEntry}.
 * @param ctx the parse tree
 */
fn enter_fileControlEntry(&mut self, _ctx: &FileControlEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileControlEntry}.
 * @param ctx the parse tree
 */
fn exit_fileControlEntry(&mut self, _ctx: &FileControlEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#selectClause}.
 * @param ctx the parse tree
 */
fn enter_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#selectClause}.
 * @param ctx the parse tree
 */
fn exit_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileControlClause}.
 * @param ctx the parse tree
 */
fn enter_fileControlClause(&mut self, _ctx: &FileControlClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileControlClause}.
 * @param ctx the parse tree
 */
fn exit_fileControlClause(&mut self, _ctx: &FileControlClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#assignClause}.
 * @param ctx the parse tree
 */
fn enter_assignClause(&mut self, _ctx: &AssignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#assignClause}.
 * @param ctx the parse tree
 */
fn exit_assignClause(&mut self, _ctx: &AssignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reserveClause}.
 * @param ctx the parse tree
 */
fn enter_reserveClause(&mut self, _ctx: &ReserveClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reserveClause}.
 * @param ctx the parse tree
 */
fn exit_reserveClause(&mut self, _ctx: &ReserveClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#organizationClause}.
 * @param ctx the parse tree
 */
fn enter_organizationClause(&mut self, _ctx: &OrganizationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#organizationClause}.
 * @param ctx the parse tree
 */
fn exit_organizationClause(&mut self, _ctx: &OrganizationClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#paddingCharacterClause}.
 * @param ctx the parse tree
 */
fn enter_paddingCharacterClause(&mut self, _ctx: &PaddingCharacterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#paddingCharacterClause}.
 * @param ctx the parse tree
 */
fn exit_paddingCharacterClause(&mut self, _ctx: &PaddingCharacterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordDelimiterClause}.
 * @param ctx the parse tree
 */
fn enter_recordDelimiterClause(&mut self, _ctx: &RecordDelimiterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordDelimiterClause}.
 * @param ctx the parse tree
 */
fn exit_recordDelimiterClause(&mut self, _ctx: &RecordDelimiterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#accessModeClause}.
 * @param ctx the parse tree
 */
fn enter_accessModeClause(&mut self, _ctx: &AccessModeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#accessModeClause}.
 * @param ctx the parse tree
 */
fn exit_accessModeClause(&mut self, _ctx: &AccessModeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordKeyClause}.
 * @param ctx the parse tree
 */
fn enter_recordKeyClause(&mut self, _ctx: &RecordKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordKeyClause}.
 * @param ctx the parse tree
 */
fn exit_recordKeyClause(&mut self, _ctx: &RecordKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alternateRecordKeyClause}.
 * @param ctx the parse tree
 */
fn enter_alternateRecordKeyClause(&mut self, _ctx: &AlternateRecordKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alternateRecordKeyClause}.
 * @param ctx the parse tree
 */
fn exit_alternateRecordKeyClause(&mut self, _ctx: &AlternateRecordKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#passwordClause}.
 * @param ctx the parse tree
 */
fn enter_passwordClause(&mut self, _ctx: &PasswordClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#passwordClause}.
 * @param ctx the parse tree
 */
fn exit_passwordClause(&mut self, _ctx: &PasswordClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileStatusClause}.
 * @param ctx the parse tree
 */
fn enter_fileStatusClause(&mut self, _ctx: &FileStatusClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileStatusClause}.
 * @param ctx the parse tree
 */
fn exit_fileStatusClause(&mut self, _ctx: &FileStatusClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relativeKeyClause}.
 * @param ctx the parse tree
 */
fn enter_relativeKeyClause(&mut self, _ctx: &RelativeKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relativeKeyClause}.
 * @param ctx the parse tree
 */
fn exit_relativeKeyClause(&mut self, _ctx: &RelativeKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#ioControlParagraph}.
 * @param ctx the parse tree
 */
fn enter_ioControlParagraph(&mut self, _ctx: &IoControlParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#ioControlParagraph}.
 * @param ctx the parse tree
 */
fn exit_ioControlParagraph(&mut self, _ctx: &IoControlParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#ioControlClause}.
 * @param ctx the parse tree
 */
fn enter_ioControlClause(&mut self, _ctx: &IoControlClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#ioControlClause}.
 * @param ctx the parse tree
 */
fn exit_ioControlClause(&mut self, _ctx: &IoControlClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rerunClause}.
 * @param ctx the parse tree
 */
fn enter_rerunClause(&mut self, _ctx: &RerunClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rerunClause}.
 * @param ctx the parse tree
 */
fn exit_rerunClause(&mut self, _ctx: &RerunClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rerunEveryRecords}.
 * @param ctx the parse tree
 */
fn enter_rerunEveryRecords(&mut self, _ctx: &RerunEveryRecordsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rerunEveryRecords}.
 * @param ctx the parse tree
 */
fn exit_rerunEveryRecords(&mut self, _ctx: &RerunEveryRecordsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rerunEveryOf}.
 * @param ctx the parse tree
 */
fn enter_rerunEveryOf(&mut self, _ctx: &RerunEveryOfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rerunEveryOf}.
 * @param ctx the parse tree
 */
fn exit_rerunEveryOf(&mut self, _ctx: &RerunEveryOfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rerunEveryClock}.
 * @param ctx the parse tree
 */
fn enter_rerunEveryClock(&mut self, _ctx: &RerunEveryClockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rerunEveryClock}.
 * @param ctx the parse tree
 */
fn exit_rerunEveryClock(&mut self, _ctx: &RerunEveryClockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sameClause}.
 * @param ctx the parse tree
 */
fn enter_sameClause(&mut self, _ctx: &SameClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sameClause}.
 * @param ctx the parse tree
 */
fn exit_sameClause(&mut self, _ctx: &SameClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multipleFileClause}.
 * @param ctx the parse tree
 */
fn enter_multipleFileClause(&mut self, _ctx: &MultipleFileClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multipleFileClause}.
 * @param ctx the parse tree
 */
fn exit_multipleFileClause(&mut self, _ctx: &MultipleFileClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multipleFilePosition}.
 * @param ctx the parse tree
 */
fn enter_multipleFilePosition(&mut self, _ctx: &MultipleFilePositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multipleFilePosition}.
 * @param ctx the parse tree
 */
fn exit_multipleFilePosition(&mut self, _ctx: &MultipleFilePositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#commitmentControlClause}.
 * @param ctx the parse tree
 */
fn enter_commitmentControlClause(&mut self, _ctx: &CommitmentControlClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#commitmentControlClause}.
 * @param ctx the parse tree
 */
fn exit_commitmentControlClause(&mut self, _ctx: &CommitmentControlClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDivision}.
 * @param ctx the parse tree
 */
fn enter_dataDivision(&mut self, _ctx: &DataDivisionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDivision}.
 * @param ctx the parse tree
 */
fn exit_dataDivision(&mut self, _ctx: &DataDivisionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDivisionSection}.
 * @param ctx the parse tree
 */
fn enter_dataDivisionSection(&mut self, _ctx: &DataDivisionSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDivisionSection}.
 * @param ctx the parse tree
 */
fn exit_dataDivisionSection(&mut self, _ctx: &DataDivisionSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileSection}.
 * @param ctx the parse tree
 */
fn enter_fileSection(&mut self, _ctx: &FileSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileSection}.
 * @param ctx the parse tree
 */
fn exit_fileSection(&mut self, _ctx: &FileSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_fileDescriptionEntry(&mut self, _ctx: &FileDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_fileDescriptionEntry(&mut self, _ctx: &FileDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileDescriptionEntryClause}.
 * @param ctx the parse tree
 */
fn enter_fileDescriptionEntryClause(&mut self, _ctx: &FileDescriptionEntryClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileDescriptionEntryClause}.
 * @param ctx the parse tree
 */
fn exit_fileDescriptionEntryClause(&mut self, _ctx: &FileDescriptionEntryClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#externalClause}.
 * @param ctx the parse tree
 */
fn enter_externalClause(&mut self, _ctx: &ExternalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#externalClause}.
 * @param ctx the parse tree
 */
fn exit_externalClause(&mut self, _ctx: &ExternalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#globalClause}.
 * @param ctx the parse tree
 */
fn enter_globalClause(&mut self, _ctx: &GlobalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#globalClause}.
 * @param ctx the parse tree
 */
fn exit_globalClause(&mut self, _ctx: &GlobalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#blockContainsClause}.
 * @param ctx the parse tree
 */
fn enter_blockContainsClause(&mut self, _ctx: &BlockContainsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#blockContainsClause}.
 * @param ctx the parse tree
 */
fn exit_blockContainsClause(&mut self, _ctx: &BlockContainsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#blockContainsTo}.
 * @param ctx the parse tree
 */
fn enter_blockContainsTo(&mut self, _ctx: &BlockContainsToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#blockContainsTo}.
 * @param ctx the parse tree
 */
fn exit_blockContainsTo(&mut self, _ctx: &BlockContainsToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordContainsClause}.
 * @param ctx the parse tree
 */
fn enter_recordContainsClause(&mut self, _ctx: &RecordContainsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordContainsClause}.
 * @param ctx the parse tree
 */
fn exit_recordContainsClause(&mut self, _ctx: &RecordContainsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat1}.
 * @param ctx the parse tree
 */
fn enter_recordContainsClauseFormat1(&mut self, _ctx: &RecordContainsClauseFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat1}.
 * @param ctx the parse tree
 */
fn exit_recordContainsClauseFormat1(&mut self, _ctx: &RecordContainsClauseFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat2}.
 * @param ctx the parse tree
 */
fn enter_recordContainsClauseFormat2(&mut self, _ctx: &RecordContainsClauseFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat2}.
 * @param ctx the parse tree
 */
fn exit_recordContainsClauseFormat2(&mut self, _ctx: &RecordContainsClauseFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat3}.
 * @param ctx the parse tree
 */
fn enter_recordContainsClauseFormat3(&mut self, _ctx: &RecordContainsClauseFormat3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordContainsClauseFormat3}.
 * @param ctx the parse tree
 */
fn exit_recordContainsClauseFormat3(&mut self, _ctx: &RecordContainsClauseFormat3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordContainsTo}.
 * @param ctx the parse tree
 */
fn enter_recordContainsTo(&mut self, _ctx: &RecordContainsToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordContainsTo}.
 * @param ctx the parse tree
 */
fn exit_recordContainsTo(&mut self, _ctx: &RecordContainsToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#labelRecordsClause}.
 * @param ctx the parse tree
 */
fn enter_labelRecordsClause(&mut self, _ctx: &LabelRecordsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#labelRecordsClause}.
 * @param ctx the parse tree
 */
fn exit_labelRecordsClause(&mut self, _ctx: &LabelRecordsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#valueOfClause}.
 * @param ctx the parse tree
 */
fn enter_valueOfClause(&mut self, _ctx: &ValueOfClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#valueOfClause}.
 * @param ctx the parse tree
 */
fn exit_valueOfClause(&mut self, _ctx: &ValueOfClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#valuePair}.
 * @param ctx the parse tree
 */
fn enter_valuePair(&mut self, _ctx: &ValuePairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#valuePair}.
 * @param ctx the parse tree
 */
fn exit_valuePair(&mut self, _ctx: &ValuePairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataRecordsClause}.
 * @param ctx the parse tree
 */
fn enter_dataRecordsClause(&mut self, _ctx: &DataRecordsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataRecordsClause}.
 * @param ctx the parse tree
 */
fn exit_dataRecordsClause(&mut self, _ctx: &DataRecordsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linageClause}.
 * @param ctx the parse tree
 */
fn enter_linageClause(&mut self, _ctx: &LinageClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linageClause}.
 * @param ctx the parse tree
 */
fn exit_linageClause(&mut self, _ctx: &LinageClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linageAt}.
 * @param ctx the parse tree
 */
fn enter_linageAt(&mut self, _ctx: &LinageAtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linageAt}.
 * @param ctx the parse tree
 */
fn exit_linageAt(&mut self, _ctx: &LinageAtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linageFootingAt}.
 * @param ctx the parse tree
 */
fn enter_linageFootingAt(&mut self, _ctx: &LinageFootingAtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linageFootingAt}.
 * @param ctx the parse tree
 */
fn exit_linageFootingAt(&mut self, _ctx: &LinageFootingAtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linageLinesAtTop}.
 * @param ctx the parse tree
 */
fn enter_linageLinesAtTop(&mut self, _ctx: &LinageLinesAtTopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linageLinesAtTop}.
 * @param ctx the parse tree
 */
fn exit_linageLinesAtTop(&mut self, _ctx: &LinageLinesAtTopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linageLinesAtBottom}.
 * @param ctx the parse tree
 */
fn enter_linageLinesAtBottom(&mut self, _ctx: &LinageLinesAtBottomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linageLinesAtBottom}.
 * @param ctx the parse tree
 */
fn exit_linageLinesAtBottom(&mut self, _ctx: &LinageLinesAtBottomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordingModeClause}.
 * @param ctx the parse tree
 */
fn enter_recordingModeClause(&mut self, _ctx: &RecordingModeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordingModeClause}.
 * @param ctx the parse tree
 */
fn exit_recordingModeClause(&mut self, _ctx: &RecordingModeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#modeStatement}.
 * @param ctx the parse tree
 */
fn enter_modeStatement(&mut self, _ctx: &ModeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#modeStatement}.
 * @param ctx the parse tree
 */
fn exit_modeStatement(&mut self, _ctx: &ModeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#codeSetClause}.
 * @param ctx the parse tree
 */
fn enter_codeSetClause(&mut self, _ctx: &CodeSetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#codeSetClause}.
 * @param ctx the parse tree
 */
fn exit_codeSetClause(&mut self, _ctx: &CodeSetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportClause}.
 * @param ctx the parse tree
 */
fn enter_reportClause(&mut self, _ctx: &ReportClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportClause}.
 * @param ctx the parse tree
 */
fn exit_reportClause(&mut self, _ctx: &ReportClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataBaseSection}.
 * @param ctx the parse tree
 */
fn enter_dataBaseSection(&mut self, _ctx: &DataBaseSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataBaseSection}.
 * @param ctx the parse tree
 */
fn exit_dataBaseSection(&mut self, _ctx: &DataBaseSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataBaseSectionEntry}.
 * @param ctx the parse tree
 */
fn enter_dataBaseSectionEntry(&mut self, _ctx: &DataBaseSectionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataBaseSectionEntry}.
 * @param ctx the parse tree
 */
fn exit_dataBaseSectionEntry(&mut self, _ctx: &DataBaseSectionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#workingStorageSection}.
 * @param ctx the parse tree
 */
fn enter_workingStorageSection(&mut self, _ctx: &WorkingStorageSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#workingStorageSection}.
 * @param ctx the parse tree
 */
fn exit_workingStorageSection(&mut self, _ctx: &WorkingStorageSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#linkageSection}.
 * @param ctx the parse tree
 */
fn enter_linkageSection(&mut self, _ctx: &LinkageSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#linkageSection}.
 * @param ctx the parse tree
 */
fn exit_linkageSection(&mut self, _ctx: &LinkageSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#communicationSection}.
 * @param ctx the parse tree
 */
fn enter_communicationSection(&mut self, _ctx: &CommunicationSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#communicationSection}.
 * @param ctx the parse tree
 */
fn exit_communicationSection(&mut self, _ctx: &CommunicationSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_communicationDescriptionEntry(&mut self, _ctx: &CommunicationDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_communicationDescriptionEntry(&mut self, _ctx: &CommunicationDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn enter_communicationDescriptionEntryFormat1(&mut self, _ctx: &CommunicationDescriptionEntryFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn exit_communicationDescriptionEntryFormat1(&mut self, _ctx: &CommunicationDescriptionEntryFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn enter_communicationDescriptionEntryFormat2(&mut self, _ctx: &CommunicationDescriptionEntryFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn exit_communicationDescriptionEntryFormat2(&mut self, _ctx: &CommunicationDescriptionEntryFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn enter_communicationDescriptionEntryFormat3(&mut self, _ctx: &CommunicationDescriptionEntryFormat3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#communicationDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn exit_communicationDescriptionEntryFormat3(&mut self, _ctx: &CommunicationDescriptionEntryFormat3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#destinationCountClause}.
 * @param ctx the parse tree
 */
fn enter_destinationCountClause(&mut self, _ctx: &DestinationCountClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#destinationCountClause}.
 * @param ctx the parse tree
 */
fn exit_destinationCountClause(&mut self, _ctx: &DestinationCountClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#destinationTableClause}.
 * @param ctx the parse tree
 */
fn enter_destinationTableClause(&mut self, _ctx: &DestinationTableClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#destinationTableClause}.
 * @param ctx the parse tree
 */
fn exit_destinationTableClause(&mut self, _ctx: &DestinationTableClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#endKeyClause}.
 * @param ctx the parse tree
 */
fn enter_endKeyClause(&mut self, _ctx: &EndKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#endKeyClause}.
 * @param ctx the parse tree
 */
fn exit_endKeyClause(&mut self, _ctx: &EndKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#errorKeyClause}.
 * @param ctx the parse tree
 */
fn enter_errorKeyClause(&mut self, _ctx: &ErrorKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#errorKeyClause}.
 * @param ctx the parse tree
 */
fn exit_errorKeyClause(&mut self, _ctx: &ErrorKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#messageCountClause}.
 * @param ctx the parse tree
 */
fn enter_messageCountClause(&mut self, _ctx: &MessageCountClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#messageCountClause}.
 * @param ctx the parse tree
 */
fn exit_messageCountClause(&mut self, _ctx: &MessageCountClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#messageDateClause}.
 * @param ctx the parse tree
 */
fn enter_messageDateClause(&mut self, _ctx: &MessageDateClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#messageDateClause}.
 * @param ctx the parse tree
 */
fn exit_messageDateClause(&mut self, _ctx: &MessageDateClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#messageTimeClause}.
 * @param ctx the parse tree
 */
fn enter_messageTimeClause(&mut self, _ctx: &MessageTimeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#messageTimeClause}.
 * @param ctx the parse tree
 */
fn exit_messageTimeClause(&mut self, _ctx: &MessageTimeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#statusKeyClause}.
 * @param ctx the parse tree
 */
fn enter_statusKeyClause(&mut self, _ctx: &StatusKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#statusKeyClause}.
 * @param ctx the parse tree
 */
fn exit_statusKeyClause(&mut self, _ctx: &StatusKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicDestinationClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicDestinationClause(&mut self, _ctx: &SymbolicDestinationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicDestinationClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicDestinationClause(&mut self, _ctx: &SymbolicDestinationClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicQueueClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicQueueClause(&mut self, _ctx: &SymbolicQueueClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicQueueClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicQueueClause(&mut self, _ctx: &SymbolicQueueClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicSourceClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicSourceClause(&mut self, _ctx: &SymbolicSourceClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicSourceClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicSourceClause(&mut self, _ctx: &SymbolicSourceClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicTerminalClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicTerminalClause(&mut self, _ctx: &SymbolicTerminalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicTerminalClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicTerminalClause(&mut self, _ctx: &SymbolicTerminalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicSubQueueClause}.
 * @param ctx the parse tree
 */
fn enter_symbolicSubQueueClause(&mut self, _ctx: &SymbolicSubQueueClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicSubQueueClause}.
 * @param ctx the parse tree
 */
fn exit_symbolicSubQueueClause(&mut self, _ctx: &SymbolicSubQueueClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#textLengthClause}.
 * @param ctx the parse tree
 */
fn enter_textLengthClause(&mut self, _ctx: &TextLengthClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#textLengthClause}.
 * @param ctx the parse tree
 */
fn exit_textLengthClause(&mut self, _ctx: &TextLengthClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#localStorageSection}.
 * @param ctx the parse tree
 */
fn enter_localStorageSection(&mut self, _ctx: &LocalStorageSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#localStorageSection}.
 * @param ctx the parse tree
 */
fn exit_localStorageSection(&mut self, _ctx: &LocalStorageSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenSection}.
 * @param ctx the parse tree
 */
fn enter_screenSection(&mut self, _ctx: &ScreenSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenSection}.
 * @param ctx the parse tree
 */
fn exit_screenSection(&mut self, _ctx: &ScreenSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionEntry(&mut self, _ctx: &ScreenDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionEntry(&mut self, _ctx: &ScreenDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionBlankClause(&mut self, _ctx: &ScreenDescriptionBlankClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionBlankClause(&mut self, _ctx: &ScreenDescriptionBlankClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionBellClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionBellClause(&mut self, _ctx: &ScreenDescriptionBellClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionBellClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionBellClause(&mut self, _ctx: &ScreenDescriptionBellClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionBlinkClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionBlinkClause(&mut self, _ctx: &ScreenDescriptionBlinkClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlinkClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionBlinkClause(&mut self, _ctx: &ScreenDescriptionBlinkClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionEraseClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionEraseClause(&mut self, _ctx: &ScreenDescriptionEraseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionEraseClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionEraseClause(&mut self, _ctx: &ScreenDescriptionEraseClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionLightClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionLightClause(&mut self, _ctx: &ScreenDescriptionLightClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionLightClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionLightClause(&mut self, _ctx: &ScreenDescriptionLightClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionGridClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionGridClause(&mut self, _ctx: &ScreenDescriptionGridClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionGridClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionGridClause(&mut self, _ctx: &ScreenDescriptionGridClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionReverseVideoClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionReverseVideoClause(&mut self, _ctx: &ScreenDescriptionReverseVideoClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionReverseVideoClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionReverseVideoClause(&mut self, _ctx: &ScreenDescriptionReverseVideoClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionUnderlineClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionUnderlineClause(&mut self, _ctx: &ScreenDescriptionUnderlineClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionUnderlineClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionUnderlineClause(&mut self, _ctx: &ScreenDescriptionUnderlineClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionSizeClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionSizeClause(&mut self, _ctx: &ScreenDescriptionSizeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionSizeClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionSizeClause(&mut self, _ctx: &ScreenDescriptionSizeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionLineClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionLineClause(&mut self, _ctx: &ScreenDescriptionLineClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionLineClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionLineClause(&mut self, _ctx: &ScreenDescriptionLineClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionColumnClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionColumnClause(&mut self, _ctx: &ScreenDescriptionColumnClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionColumnClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionColumnClause(&mut self, _ctx: &ScreenDescriptionColumnClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionForegroundColorClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionForegroundColorClause(&mut self, _ctx: &ScreenDescriptionForegroundColorClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionForegroundColorClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionForegroundColorClause(&mut self, _ctx: &ScreenDescriptionForegroundColorClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionBackgroundColorClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionBackgroundColorClause(&mut self, _ctx: &ScreenDescriptionBackgroundColorClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionBackgroundColorClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionBackgroundColorClause(&mut self, _ctx: &ScreenDescriptionBackgroundColorClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionControlClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionControlClause(&mut self, _ctx: &ScreenDescriptionControlClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionControlClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionControlClause(&mut self, _ctx: &ScreenDescriptionControlClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionValueClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionValueClause(&mut self, _ctx: &ScreenDescriptionValueClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionValueClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionValueClause(&mut self, _ctx: &ScreenDescriptionValueClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionPictureClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionPictureClause(&mut self, _ctx: &ScreenDescriptionPictureClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionPictureClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionPictureClause(&mut self, _ctx: &ScreenDescriptionPictureClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionFromClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionFromClause(&mut self, _ctx: &ScreenDescriptionFromClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionFromClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionFromClause(&mut self, _ctx: &ScreenDescriptionFromClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionToClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionToClause(&mut self, _ctx: &ScreenDescriptionToClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionToClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionToClause(&mut self, _ctx: &ScreenDescriptionToClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionUsingClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionUsingClause(&mut self, _ctx: &ScreenDescriptionUsingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsingClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionUsingClause(&mut self, _ctx: &ScreenDescriptionUsingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionUsageClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionUsageClause(&mut self, _ctx: &ScreenDescriptionUsageClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionUsageClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionUsageClause(&mut self, _ctx: &ScreenDescriptionUsageClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionBlankWhenZeroClause(&mut self, _ctx: &ScreenDescriptionBlankWhenZeroClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionBlankWhenZeroClause(&mut self, _ctx: &ScreenDescriptionBlankWhenZeroClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionJustifiedClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionJustifiedClause(&mut self, _ctx: &ScreenDescriptionJustifiedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionJustifiedClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionJustifiedClause(&mut self, _ctx: &ScreenDescriptionJustifiedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionSignClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionSignClause(&mut self, _ctx: &ScreenDescriptionSignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionSignClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionSignClause(&mut self, _ctx: &ScreenDescriptionSignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionAutoClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionAutoClause(&mut self, _ctx: &ScreenDescriptionAutoClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionAutoClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionAutoClause(&mut self, _ctx: &ScreenDescriptionAutoClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionSecureClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionSecureClause(&mut self, _ctx: &ScreenDescriptionSecureClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionSecureClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionSecureClause(&mut self, _ctx: &ScreenDescriptionSecureClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionRequiredClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionRequiredClause(&mut self, _ctx: &ScreenDescriptionRequiredClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionRequiredClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionRequiredClause(&mut self, _ctx: &ScreenDescriptionRequiredClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionPromptClause(&mut self, _ctx: &ScreenDescriptionPromptClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionPromptClause(&mut self, _ctx: &ScreenDescriptionPromptClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptOccursClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionPromptOccursClause(&mut self, _ctx: &ScreenDescriptionPromptOccursClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionPromptOccursClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionPromptOccursClause(&mut self, _ctx: &ScreenDescriptionPromptOccursClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionFullClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionFullClause(&mut self, _ctx: &ScreenDescriptionFullClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionFullClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionFullClause(&mut self, _ctx: &ScreenDescriptionFullClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenDescriptionZeroFillClause}.
 * @param ctx the parse tree
 */
fn enter_screenDescriptionZeroFillClause(&mut self, _ctx: &ScreenDescriptionZeroFillClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenDescriptionZeroFillClause}.
 * @param ctx the parse tree
 */
fn exit_screenDescriptionZeroFillClause(&mut self, _ctx: &ScreenDescriptionZeroFillClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportSection}.
 * @param ctx the parse tree
 */
fn enter_reportSection(&mut self, _ctx: &ReportSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportSection}.
 * @param ctx the parse tree
 */
fn exit_reportSection(&mut self, _ctx: &ReportSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescription}.
 * @param ctx the parse tree
 */
fn enter_reportDescription(&mut self, _ctx: &ReportDescriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescription}.
 * @param ctx the parse tree
 */
fn exit_reportDescription(&mut self, _ctx: &ReportDescriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionEntry(&mut self, _ctx: &ReportDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionEntry(&mut self, _ctx: &ReportDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionGlobalClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionGlobalClause(&mut self, _ctx: &ReportDescriptionGlobalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionGlobalClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionGlobalClause(&mut self, _ctx: &ReportDescriptionGlobalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionPageLimitClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionPageLimitClause(&mut self, _ctx: &ReportDescriptionPageLimitClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionPageLimitClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionPageLimitClause(&mut self, _ctx: &ReportDescriptionPageLimitClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionHeadingClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionHeadingClause(&mut self, _ctx: &ReportDescriptionHeadingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionHeadingClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionHeadingClause(&mut self, _ctx: &ReportDescriptionHeadingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionFirstDetailClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionFirstDetailClause(&mut self, _ctx: &ReportDescriptionFirstDetailClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionFirstDetailClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionFirstDetailClause(&mut self, _ctx: &ReportDescriptionFirstDetailClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionLastDetailClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionLastDetailClause(&mut self, _ctx: &ReportDescriptionLastDetailClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionLastDetailClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionLastDetailClause(&mut self, _ctx: &ReportDescriptionLastDetailClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportDescriptionFootingClause}.
 * @param ctx the parse tree
 */
fn enter_reportDescriptionFootingClause(&mut self, _ctx: &ReportDescriptionFootingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportDescriptionFootingClause}.
 * @param ctx the parse tree
 */
fn exit_reportDescriptionFootingClause(&mut self, _ctx: &ReportDescriptionFootingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_reportGroupDescriptionEntry(&mut self, _ctx: &ReportGroupDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_reportGroupDescriptionEntry(&mut self, _ctx: &ReportGroupDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn enter_reportGroupDescriptionEntryFormat1(&mut self, _ctx: &ReportGroupDescriptionEntryFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn exit_reportGroupDescriptionEntryFormat1(&mut self, _ctx: &ReportGroupDescriptionEntryFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn enter_reportGroupDescriptionEntryFormat2(&mut self, _ctx: &ReportGroupDescriptionEntryFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn exit_reportGroupDescriptionEntryFormat2(&mut self, _ctx: &ReportGroupDescriptionEntryFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn enter_reportGroupDescriptionEntryFormat3(&mut self, _ctx: &ReportGroupDescriptionEntryFormat3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn exit_reportGroupDescriptionEntryFormat3(&mut self, _ctx: &ReportGroupDescriptionEntryFormat3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupBlankWhenZeroClause(&mut self, _ctx: &ReportGroupBlankWhenZeroClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupBlankWhenZeroClause(&mut self, _ctx: &ReportGroupBlankWhenZeroClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupColumnNumberClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupColumnNumberClause(&mut self, _ctx: &ReportGroupColumnNumberClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupColumnNumberClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupColumnNumberClause(&mut self, _ctx: &ReportGroupColumnNumberClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupIndicateClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupIndicateClause(&mut self, _ctx: &ReportGroupIndicateClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupIndicateClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupIndicateClause(&mut self, _ctx: &ReportGroupIndicateClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupJustifiedClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupJustifiedClause(&mut self, _ctx: &ReportGroupJustifiedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupJustifiedClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupJustifiedClause(&mut self, _ctx: &ReportGroupJustifiedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupLineNumberClause(&mut self, _ctx: &ReportGroupLineNumberClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupLineNumberClause(&mut self, _ctx: &ReportGroupLineNumberClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberNextPage}.
 * @param ctx the parse tree
 */
fn enter_reportGroupLineNumberNextPage(&mut self, _ctx: &ReportGroupLineNumberNextPageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberNextPage}.
 * @param ctx the parse tree
 */
fn exit_reportGroupLineNumberNextPage(&mut self, _ctx: &ReportGroupLineNumberNextPageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberPlus}.
 * @param ctx the parse tree
 */
fn enter_reportGroupLineNumberPlus(&mut self, _ctx: &ReportGroupLineNumberPlusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupLineNumberPlus}.
 * @param ctx the parse tree
 */
fn exit_reportGroupLineNumberPlus(&mut self, _ctx: &ReportGroupLineNumberPlusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupNextGroupClause(&mut self, _ctx: &ReportGroupNextGroupClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupNextGroupClause(&mut self, _ctx: &ReportGroupNextGroupClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupPlus}.
 * @param ctx the parse tree
 */
fn enter_reportGroupNextGroupPlus(&mut self, _ctx: &ReportGroupNextGroupPlusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupPlus}.
 * @param ctx the parse tree
 */
fn exit_reportGroupNextGroupPlus(&mut self, _ctx: &ReportGroupNextGroupPlusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupNextPage}.
 * @param ctx the parse tree
 */
fn enter_reportGroupNextGroupNextPage(&mut self, _ctx: &ReportGroupNextGroupNextPageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupNextGroupNextPage}.
 * @param ctx the parse tree
 */
fn exit_reportGroupNextGroupNextPage(&mut self, _ctx: &ReportGroupNextGroupNextPageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupPictureClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupPictureClause(&mut self, _ctx: &ReportGroupPictureClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupPictureClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupPictureClause(&mut self, _ctx: &ReportGroupPictureClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupResetClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupResetClause(&mut self, _ctx: &ReportGroupResetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupResetClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupResetClause(&mut self, _ctx: &ReportGroupResetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupSignClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupSignClause(&mut self, _ctx: &ReportGroupSignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupSignClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupSignClause(&mut self, _ctx: &ReportGroupSignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupSourceClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupSourceClause(&mut self, _ctx: &ReportGroupSourceClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupSourceClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupSourceClause(&mut self, _ctx: &ReportGroupSourceClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupSumClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupSumClause(&mut self, _ctx: &ReportGroupSumClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupSumClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupSumClause(&mut self, _ctx: &ReportGroupSumClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeClause(&mut self, _ctx: &ReportGroupTypeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeClause(&mut self, _ctx: &ReportGroupTypeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportHeading}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeReportHeading(&mut self, _ctx: &ReportGroupTypeReportHeadingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportHeading}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeReportHeading(&mut self, _ctx: &ReportGroupTypeReportHeadingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypePageHeading}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypePageHeading(&mut self, _ctx: &ReportGroupTypePageHeadingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageHeading}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypePageHeading(&mut self, _ctx: &ReportGroupTypePageHeadingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlHeading}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeControlHeading(&mut self, _ctx: &ReportGroupTypeControlHeadingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlHeading}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeControlHeading(&mut self, _ctx: &ReportGroupTypeControlHeadingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeDetail}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeDetail(&mut self, _ctx: &ReportGroupTypeDetailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeDetail}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeDetail(&mut self, _ctx: &ReportGroupTypeDetailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlFooting}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeControlFooting(&mut self, _ctx: &ReportGroupTypeControlFootingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeControlFooting}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeControlFooting(&mut self, _ctx: &ReportGroupTypeControlFootingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupUsageClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupUsageClause(&mut self, _ctx: &ReportGroupUsageClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupUsageClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupUsageClause(&mut self, _ctx: &ReportGroupUsageClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypePageFooting}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypePageFooting(&mut self, _ctx: &ReportGroupTypePageFootingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypePageFooting}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypePageFooting(&mut self, _ctx: &ReportGroupTypePageFootingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportFooting}.
 * @param ctx the parse tree
 */
fn enter_reportGroupTypeReportFooting(&mut self, _ctx: &ReportGroupTypeReportFootingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupTypeReportFooting}.
 * @param ctx the parse tree
 */
fn exit_reportGroupTypeReportFooting(&mut self, _ctx: &ReportGroupTypeReportFootingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportGroupValueClause}.
 * @param ctx the parse tree
 */
fn enter_reportGroupValueClause(&mut self, _ctx: &ReportGroupValueClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportGroupValueClause}.
 * @param ctx the parse tree
 */
fn exit_reportGroupValueClause(&mut self, _ctx: &ReportGroupValueClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#programLibrarySection}.
 * @param ctx the parse tree
 */
fn enter_programLibrarySection(&mut self, _ctx: &ProgramLibrarySectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#programLibrarySection}.
 * @param ctx the parse tree
 */
fn exit_programLibrarySection(&mut self, _ctx: &ProgramLibrarySectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_libraryDescriptionEntry(&mut self, _ctx: &LibraryDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_libraryDescriptionEntry(&mut self, _ctx: &LibraryDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn enter_libraryDescriptionEntryFormat1(&mut self, _ctx: &LibraryDescriptionEntryFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn exit_libraryDescriptionEntryFormat1(&mut self, _ctx: &LibraryDescriptionEntryFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn enter_libraryDescriptionEntryFormat2(&mut self, _ctx: &LibraryDescriptionEntryFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn exit_libraryDescriptionEntryFormat2(&mut self, _ctx: &LibraryDescriptionEntryFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat1}.
 * @param ctx the parse tree
 */
fn enter_libraryAttributeClauseFormat1(&mut self, _ctx: &LibraryAttributeClauseFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat1}.
 * @param ctx the parse tree
 */
fn exit_libraryAttributeClauseFormat1(&mut self, _ctx: &LibraryAttributeClauseFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat2}.
 * @param ctx the parse tree
 */
fn enter_libraryAttributeClauseFormat2(&mut self, _ctx: &LibraryAttributeClauseFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryAttributeClauseFormat2}.
 * @param ctx the parse tree
 */
fn exit_libraryAttributeClauseFormat2(&mut self, _ctx: &LibraryAttributeClauseFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryAttributeFunction}.
 * @param ctx the parse tree
 */
fn enter_libraryAttributeFunction(&mut self, _ctx: &LibraryAttributeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryAttributeFunction}.
 * @param ctx the parse tree
 */
fn exit_libraryAttributeFunction(&mut self, _ctx: &LibraryAttributeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryAttributeParameter}.
 * @param ctx the parse tree
 */
fn enter_libraryAttributeParameter(&mut self, _ctx: &LibraryAttributeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryAttributeParameter}.
 * @param ctx the parse tree
 */
fn exit_libraryAttributeParameter(&mut self, _ctx: &LibraryAttributeParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryAttributeTitle}.
 * @param ctx the parse tree
 */
fn enter_libraryAttributeTitle(&mut self, _ctx: &LibraryAttributeTitleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryAttributeTitle}.
 * @param ctx the parse tree
 */
fn exit_libraryAttributeTitle(&mut self, _ctx: &LibraryAttributeTitleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat1}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureClauseFormat1(&mut self, _ctx: &LibraryEntryProcedureClauseFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat1}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureClauseFormat1(&mut self, _ctx: &LibraryEntryProcedureClauseFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat2}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureClauseFormat2(&mut self, _ctx: &LibraryEntryProcedureClauseFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureClauseFormat2}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureClauseFormat2(&mut self, _ctx: &LibraryEntryProcedureClauseFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureForClause}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureForClause(&mut self, _ctx: &LibraryEntryProcedureForClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureForClause}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureForClause(&mut self, _ctx: &LibraryEntryProcedureForClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureGivingClause}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureGivingClause(&mut self, _ctx: &LibraryEntryProcedureGivingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureGivingClause}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureGivingClause(&mut self, _ctx: &LibraryEntryProcedureGivingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingClause}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureUsingClause(&mut self, _ctx: &LibraryEntryProcedureUsingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingClause}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureUsingClause(&mut self, _ctx: &LibraryEntryProcedureUsingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingName}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureUsingName(&mut self, _ctx: &LibraryEntryProcedureUsingNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureUsingName}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureUsingName(&mut self, _ctx: &LibraryEntryProcedureUsingNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithClause}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureWithClause(&mut self, _ctx: &LibraryEntryProcedureWithClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithClause}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureWithClause(&mut self, _ctx: &LibraryEntryProcedureWithClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithName}.
 * @param ctx the parse tree
 */
fn enter_libraryEntryProcedureWithName(&mut self, _ctx: &LibraryEntryProcedureWithNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryEntryProcedureWithName}.
 * @param ctx the parse tree
 */
fn exit_libraryEntryProcedureWithName(&mut self, _ctx: &LibraryEntryProcedureWithNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryIsCommonClause}.
 * @param ctx the parse tree
 */
fn enter_libraryIsCommonClause(&mut self, _ctx: &LibraryIsCommonClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryIsCommonClause}.
 * @param ctx the parse tree
 */
fn exit_libraryIsCommonClause(&mut self, _ctx: &LibraryIsCommonClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryIsGlobalClause}.
 * @param ctx the parse tree
 */
fn enter_libraryIsGlobalClause(&mut self, _ctx: &LibraryIsGlobalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryIsGlobalClause}.
 * @param ctx the parse tree
 */
fn exit_libraryIsGlobalClause(&mut self, _ctx: &LibraryIsGlobalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescriptionEntry}.
 * @param ctx the parse tree
 */
fn enter_dataDescriptionEntry(&mut self, _ctx: &DataDescriptionEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntry}.
 * @param ctx the parse tree
 */
fn exit_dataDescriptionEntry(&mut self, _ctx: &DataDescriptionEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn enter_dataDescriptionEntryFormat1(&mut self, _ctx: &DataDescriptionEntryFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat1}.
 * @param ctx the parse tree
 */
fn exit_dataDescriptionEntryFormat1(&mut self, _ctx: &DataDescriptionEntryFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn enter_dataDescriptionEntryFormat2(&mut self, _ctx: &DataDescriptionEntryFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat2}.
 * @param ctx the parse tree
 */
fn exit_dataDescriptionEntryFormat2(&mut self, _ctx: &DataDescriptionEntryFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn enter_dataDescriptionEntryFormat3(&mut self, _ctx: &DataDescriptionEntryFormat3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryFormat3}.
 * @param ctx the parse tree
 */
fn exit_dataDescriptionEntryFormat3(&mut self, _ctx: &DataDescriptionEntryFormat3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryExecSql}.
 * @param ctx the parse tree
 */
fn enter_dataDescriptionEntryExecSql(&mut self, _ctx: &DataDescriptionEntryExecSqlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescriptionEntryExecSql}.
 * @param ctx the parse tree
 */
fn exit_dataDescriptionEntryExecSql(&mut self, _ctx: &DataDescriptionEntryExecSqlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataAlignedClause}.
 * @param ctx the parse tree
 */
fn enter_dataAlignedClause(&mut self, _ctx: &DataAlignedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataAlignedClause}.
 * @param ctx the parse tree
 */
fn exit_dataAlignedClause(&mut self, _ctx: &DataAlignedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn enter_dataBlankWhenZeroClause(&mut self, _ctx: &DataBlankWhenZeroClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataBlankWhenZeroClause}.
 * @param ctx the parse tree
 */
fn exit_dataBlankWhenZeroClause(&mut self, _ctx: &DataBlankWhenZeroClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataCommonOwnLocalClause}.
 * @param ctx the parse tree
 */
fn enter_dataCommonOwnLocalClause(&mut self, _ctx: &DataCommonOwnLocalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataCommonOwnLocalClause}.
 * @param ctx the parse tree
 */
fn exit_dataCommonOwnLocalClause(&mut self, _ctx: &DataCommonOwnLocalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataExternalClause}.
 * @param ctx the parse tree
 */
fn enter_dataExternalClause(&mut self, _ctx: &DataExternalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataExternalClause}.
 * @param ctx the parse tree
 */
fn exit_dataExternalClause(&mut self, _ctx: &DataExternalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataGlobalClause}.
 * @param ctx the parse tree
 */
fn enter_dataGlobalClause(&mut self, _ctx: &DataGlobalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataGlobalClause}.
 * @param ctx the parse tree
 */
fn exit_dataGlobalClause(&mut self, _ctx: &DataGlobalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataIntegerStringClause}.
 * @param ctx the parse tree
 */
fn enter_dataIntegerStringClause(&mut self, _ctx: &DataIntegerStringClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataIntegerStringClause}.
 * @param ctx the parse tree
 */
fn exit_dataIntegerStringClause(&mut self, _ctx: &DataIntegerStringClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataJustifiedClause}.
 * @param ctx the parse tree
 */
fn enter_dataJustifiedClause(&mut self, _ctx: &DataJustifiedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataJustifiedClause}.
 * @param ctx the parse tree
 */
fn exit_dataJustifiedClause(&mut self, _ctx: &DataJustifiedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataOccursClause}.
 * @param ctx the parse tree
 */
fn enter_dataOccursClause(&mut self, _ctx: &DataOccursClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataOccursClause}.
 * @param ctx the parse tree
 */
fn exit_dataOccursClause(&mut self, _ctx: &DataOccursClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataOccursTo}.
 * @param ctx the parse tree
 */
fn enter_dataOccursTo(&mut self, _ctx: &DataOccursToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataOccursTo}.
 * @param ctx the parse tree
 */
fn exit_dataOccursTo(&mut self, _ctx: &DataOccursToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataOccursSort}.
 * @param ctx the parse tree
 */
fn enter_dataOccursSort(&mut self, _ctx: &DataOccursSortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataOccursSort}.
 * @param ctx the parse tree
 */
fn exit_dataOccursSort(&mut self, _ctx: &DataOccursSortContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataPictureClause}.
 * @param ctx the parse tree
 */
fn enter_dataPictureClause(&mut self, _ctx: &DataPictureClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataPictureClause}.
 * @param ctx the parse tree
 */
fn exit_dataPictureClause(&mut self, _ctx: &DataPictureClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#pictureString}.
 * @param ctx the parse tree
 */
fn enter_pictureString(&mut self, _ctx: &PictureStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#pictureString}.
 * @param ctx the parse tree
 */
fn exit_pictureString(&mut self, _ctx: &PictureStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#pictureChars}.
 * @param ctx the parse tree
 */
fn enter_pictureChars(&mut self, _ctx: &PictureCharsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#pictureChars}.
 * @param ctx the parse tree
 */
fn exit_pictureChars(&mut self, _ctx: &PictureCharsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#pictureCardinality}.
 * @param ctx the parse tree
 */
fn enter_pictureCardinality(&mut self, _ctx: &PictureCardinalityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#pictureCardinality}.
 * @param ctx the parse tree
 */
fn exit_pictureCardinality(&mut self, _ctx: &PictureCardinalityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataReceivedByClause}.
 * @param ctx the parse tree
 */
fn enter_dataReceivedByClause(&mut self, _ctx: &DataReceivedByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataReceivedByClause}.
 * @param ctx the parse tree
 */
fn exit_dataReceivedByClause(&mut self, _ctx: &DataReceivedByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataRecordAreaClause}.
 * @param ctx the parse tree
 */
fn enter_dataRecordAreaClause(&mut self, _ctx: &DataRecordAreaClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataRecordAreaClause}.
 * @param ctx the parse tree
 */
fn exit_dataRecordAreaClause(&mut self, _ctx: &DataRecordAreaClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataRedefinesClause}.
 * @param ctx the parse tree
 */
fn enter_dataRedefinesClause(&mut self, _ctx: &DataRedefinesClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataRedefinesClause}.
 * @param ctx the parse tree
 */
fn exit_dataRedefinesClause(&mut self, _ctx: &DataRedefinesClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataRenamesClause}.
 * @param ctx the parse tree
 */
fn enter_dataRenamesClause(&mut self, _ctx: &DataRenamesClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataRenamesClause}.
 * @param ctx the parse tree
 */
fn exit_dataRenamesClause(&mut self, _ctx: &DataRenamesClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataSignClause}.
 * @param ctx the parse tree
 */
fn enter_dataSignClause(&mut self, _ctx: &DataSignClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataSignClause}.
 * @param ctx the parse tree
 */
fn exit_dataSignClause(&mut self, _ctx: &DataSignClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataSynchronizedClause}.
 * @param ctx the parse tree
 */
fn enter_dataSynchronizedClause(&mut self, _ctx: &DataSynchronizedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataSynchronizedClause}.
 * @param ctx the parse tree
 */
fn exit_dataSynchronizedClause(&mut self, _ctx: &DataSynchronizedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataThreadLocalClause}.
 * @param ctx the parse tree
 */
fn enter_dataThreadLocalClause(&mut self, _ctx: &DataThreadLocalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataThreadLocalClause}.
 * @param ctx the parse tree
 */
fn exit_dataThreadLocalClause(&mut self, _ctx: &DataThreadLocalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataTypeClause}.
 * @param ctx the parse tree
 */
fn enter_dataTypeClause(&mut self, _ctx: &DataTypeClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataTypeClause}.
 * @param ctx the parse tree
 */
fn exit_dataTypeClause(&mut self, _ctx: &DataTypeClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataTypeDefClause}.
 * @param ctx the parse tree
 */
fn enter_dataTypeDefClause(&mut self, _ctx: &DataTypeDefClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataTypeDefClause}.
 * @param ctx the parse tree
 */
fn exit_dataTypeDefClause(&mut self, _ctx: &DataTypeDefClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataUsageClause}.
 * @param ctx the parse tree
 */
fn enter_dataUsageClause(&mut self, _ctx: &DataUsageClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataUsageClause}.
 * @param ctx the parse tree
 */
fn exit_dataUsageClause(&mut self, _ctx: &DataUsageClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataUsingClause}.
 * @param ctx the parse tree
 */
fn enter_dataUsingClause(&mut self, _ctx: &DataUsingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataUsingClause}.
 * @param ctx the parse tree
 */
fn exit_dataUsingClause(&mut self, _ctx: &DataUsingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataValueClause}.
 * @param ctx the parse tree
 */
fn enter_dataValueClause(&mut self, _ctx: &DataValueClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataValueClause}.
 * @param ctx the parse tree
 */
fn exit_dataValueClause(&mut self, _ctx: &DataValueClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataValueInterval}.
 * @param ctx the parse tree
 */
fn enter_dataValueInterval(&mut self, _ctx: &DataValueIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataValueInterval}.
 * @param ctx the parse tree
 */
fn exit_dataValueInterval(&mut self, _ctx: &DataValueIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataValueIntervalFrom}.
 * @param ctx the parse tree
 */
fn enter_dataValueIntervalFrom(&mut self, _ctx: &DataValueIntervalFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataValueIntervalFrom}.
 * @param ctx the parse tree
 */
fn exit_dataValueIntervalFrom(&mut self, _ctx: &DataValueIntervalFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataValueIntervalTo}.
 * @param ctx the parse tree
 */
fn enter_dataValueIntervalTo(&mut self, _ctx: &DataValueIntervalToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataValueIntervalTo}.
 * @param ctx the parse tree
 */
fn exit_dataValueIntervalTo(&mut self, _ctx: &DataValueIntervalToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataWithLowerBoundsClause}.
 * @param ctx the parse tree
 */
fn enter_dataWithLowerBoundsClause(&mut self, _ctx: &DataWithLowerBoundsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataWithLowerBoundsClause}.
 * @param ctx the parse tree
 */
fn exit_dataWithLowerBoundsClause(&mut self, _ctx: &DataWithLowerBoundsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivision}.
 * @param ctx the parse tree
 */
fn enter_procedureDivision(&mut self, _ctx: &ProcedureDivisionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivision}.
 * @param ctx the parse tree
 */
fn exit_procedureDivision(&mut self, _ctx: &ProcedureDivisionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingClause}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionUsingClause(&mut self, _ctx: &ProcedureDivisionUsingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingClause}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionUsingClause(&mut self, _ctx: &ProcedureDivisionUsingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionGivingClause}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionGivingClause(&mut self, _ctx: &ProcedureDivisionGivingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionGivingClause}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionGivingClause(&mut self, _ctx: &ProcedureDivisionGivingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingParameter}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionUsingParameter(&mut self, _ctx: &ProcedureDivisionUsingParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionUsingParameter}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionUsingParameter(&mut self, _ctx: &ProcedureDivisionUsingParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionByReferencePhrase}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionByReferencePhrase(&mut self, _ctx: &ProcedureDivisionByReferencePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReferencePhrase}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionByReferencePhrase(&mut self, _ctx: &ProcedureDivisionByReferencePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionByReference}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionByReference(&mut self, _ctx: &ProcedureDivisionByReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionByReference}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionByReference(&mut self, _ctx: &ProcedureDivisionByReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionByValuePhrase}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionByValuePhrase(&mut self, _ctx: &ProcedureDivisionByValuePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValuePhrase}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionByValuePhrase(&mut self, _ctx: &ProcedureDivisionByValuePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionByValue}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionByValue(&mut self, _ctx: &ProcedureDivisionByValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionByValue}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionByValue(&mut self, _ctx: &ProcedureDivisionByValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDeclaratives}.
 * @param ctx the parse tree
 */
fn enter_procedureDeclaratives(&mut self, _ctx: &ProcedureDeclarativesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDeclaratives}.
 * @param ctx the parse tree
 */
fn exit_procedureDeclaratives(&mut self, _ctx: &ProcedureDeclarativesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDeclarative}.
 * @param ctx the parse tree
 */
fn enter_procedureDeclarative(&mut self, _ctx: &ProcedureDeclarativeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDeclarative}.
 * @param ctx the parse tree
 */
fn exit_procedureDeclarative(&mut self, _ctx: &ProcedureDeclarativeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureSectionHeader}.
 * @param ctx the parse tree
 */
fn enter_procedureSectionHeader(&mut self, _ctx: &ProcedureSectionHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureSectionHeader}.
 * @param ctx the parse tree
 */
fn exit_procedureSectionHeader(&mut self, _ctx: &ProcedureSectionHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureDivisionBody}.
 * @param ctx the parse tree
 */
fn enter_procedureDivisionBody(&mut self, _ctx: &ProcedureDivisionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureDivisionBody}.
 * @param ctx the parse tree
 */
fn exit_procedureDivisionBody(&mut self, _ctx: &ProcedureDivisionBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureSection}.
 * @param ctx the parse tree
 */
fn enter_procedureSection(&mut self, _ctx: &ProcedureSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureSection}.
 * @param ctx the parse tree
 */
fn exit_procedureSection(&mut self, _ctx: &ProcedureSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#paragraphs}.
 * @param ctx the parse tree
 */
fn enter_paragraphs(&mut self, _ctx: &ParagraphsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#paragraphs}.
 * @param ctx the parse tree
 */
fn exit_paragraphs(&mut self, _ctx: &ParagraphsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#paragraph}.
 * @param ctx the parse tree
 */
fn enter_paragraph(&mut self, _ctx: &ParagraphContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#paragraph}.
 * @param ctx the parse tree
 */
fn exit_paragraph(&mut self, _ctx: &ParagraphContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sentence}.
 * @param ctx the parse tree
 */
fn enter_sentence(&mut self, _ctx: &SentenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sentence}.
 * @param ctx the parse tree
 */
fn exit_sentence(&mut self, _ctx: &SentenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#acceptStatement}.
 * @param ctx the parse tree
 */
fn enter_acceptStatement(&mut self, _ctx: &AcceptStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#acceptStatement}.
 * @param ctx the parse tree
 */
fn exit_acceptStatement(&mut self, _ctx: &AcceptStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#acceptFromDateStatement}.
 * @param ctx the parse tree
 */
fn enter_acceptFromDateStatement(&mut self, _ctx: &AcceptFromDateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#acceptFromDateStatement}.
 * @param ctx the parse tree
 */
fn exit_acceptFromDateStatement(&mut self, _ctx: &AcceptFromDateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#acceptFromMnemonicStatement}.
 * @param ctx the parse tree
 */
fn enter_acceptFromMnemonicStatement(&mut self, _ctx: &AcceptFromMnemonicStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#acceptFromMnemonicStatement}.
 * @param ctx the parse tree
 */
fn exit_acceptFromMnemonicStatement(&mut self, _ctx: &AcceptFromMnemonicStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#acceptFromEscapeKeyStatement}.
 * @param ctx the parse tree
 */
fn enter_acceptFromEscapeKeyStatement(&mut self, _ctx: &AcceptFromEscapeKeyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#acceptFromEscapeKeyStatement}.
 * @param ctx the parse tree
 */
fn exit_acceptFromEscapeKeyStatement(&mut self, _ctx: &AcceptFromEscapeKeyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#acceptMessageCountStatement}.
 * @param ctx the parse tree
 */
fn enter_acceptMessageCountStatement(&mut self, _ctx: &AcceptMessageCountStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#acceptMessageCountStatement}.
 * @param ctx the parse tree
 */
fn exit_acceptMessageCountStatement(&mut self, _ctx: &AcceptMessageCountStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addStatement}.
 * @param ctx the parse tree
 */
fn enter_addStatement(&mut self, _ctx: &AddStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addStatement}.
 * @param ctx the parse tree
 */
fn exit_addStatement(&mut self, _ctx: &AddStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addToStatement}.
 * @param ctx the parse tree
 */
fn enter_addToStatement(&mut self, _ctx: &AddToStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addToStatement}.
 * @param ctx the parse tree
 */
fn exit_addToStatement(&mut self, _ctx: &AddToStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addToGivingStatement}.
 * @param ctx the parse tree
 */
fn enter_addToGivingStatement(&mut self, _ctx: &AddToGivingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addToGivingStatement}.
 * @param ctx the parse tree
 */
fn exit_addToGivingStatement(&mut self, _ctx: &AddToGivingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addCorrespondingStatement}.
 * @param ctx the parse tree
 */
fn enter_addCorrespondingStatement(&mut self, _ctx: &AddCorrespondingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addCorrespondingStatement}.
 * @param ctx the parse tree
 */
fn exit_addCorrespondingStatement(&mut self, _ctx: &AddCorrespondingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addFrom}.
 * @param ctx the parse tree
 */
fn enter_addFrom(&mut self, _ctx: &AddFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addFrom}.
 * @param ctx the parse tree
 */
fn exit_addFrom(&mut self, _ctx: &AddFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addTo}.
 * @param ctx the parse tree
 */
fn enter_addTo(&mut self, _ctx: &AddToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addTo}.
 * @param ctx the parse tree
 */
fn exit_addTo(&mut self, _ctx: &AddToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addToGiving}.
 * @param ctx the parse tree
 */
fn enter_addToGiving(&mut self, _ctx: &AddToGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addToGiving}.
 * @param ctx the parse tree
 */
fn exit_addToGiving(&mut self, _ctx: &AddToGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#addGiving}.
 * @param ctx the parse tree
 */
fn enter_addGiving(&mut self, _ctx: &AddGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#addGiving}.
 * @param ctx the parse tree
 */
fn exit_addGiving(&mut self, _ctx: &AddGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alteredGoTo}.
 * @param ctx the parse tree
 */
fn enter_alteredGoTo(&mut self, _ctx: &AlteredGoToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alteredGoTo}.
 * @param ctx the parse tree
 */
fn exit_alteredGoTo(&mut self, _ctx: &AlteredGoToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alterStatement}.
 * @param ctx the parse tree
 */
fn enter_alterStatement(&mut self, _ctx: &AlterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alterStatement}.
 * @param ctx the parse tree
 */
fn exit_alterStatement(&mut self, _ctx: &AlterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alterProceedTo}.
 * @param ctx the parse tree
 */
fn enter_alterProceedTo(&mut self, _ctx: &AlterProceedToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alterProceedTo}.
 * @param ctx the parse tree
 */
fn exit_alterProceedTo(&mut self, _ctx: &AlterProceedToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callStatement}.
 * @param ctx the parse tree
 */
fn enter_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callStatement}.
 * @param ctx the parse tree
 */
fn exit_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callUsingPhrase}.
 * @param ctx the parse tree
 */
fn enter_callUsingPhrase(&mut self, _ctx: &CallUsingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callUsingPhrase}.
 * @param ctx the parse tree
 */
fn exit_callUsingPhrase(&mut self, _ctx: &CallUsingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callUsingParameter}.
 * @param ctx the parse tree
 */
fn enter_callUsingParameter(&mut self, _ctx: &CallUsingParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callUsingParameter}.
 * @param ctx the parse tree
 */
fn exit_callUsingParameter(&mut self, _ctx: &CallUsingParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByReferencePhrase}.
 * @param ctx the parse tree
 */
fn enter_callByReferencePhrase(&mut self, _ctx: &CallByReferencePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByReferencePhrase}.
 * @param ctx the parse tree
 */
fn exit_callByReferencePhrase(&mut self, _ctx: &CallByReferencePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByReference}.
 * @param ctx the parse tree
 */
fn enter_callByReference(&mut self, _ctx: &CallByReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByReference}.
 * @param ctx the parse tree
 */
fn exit_callByReference(&mut self, _ctx: &CallByReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByValuePhrase}.
 * @param ctx the parse tree
 */
fn enter_callByValuePhrase(&mut self, _ctx: &CallByValuePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByValuePhrase}.
 * @param ctx the parse tree
 */
fn exit_callByValuePhrase(&mut self, _ctx: &CallByValuePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByValue}.
 * @param ctx the parse tree
 */
fn enter_callByValue(&mut self, _ctx: &CallByValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByValue}.
 * @param ctx the parse tree
 */
fn exit_callByValue(&mut self, _ctx: &CallByValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByContentPhrase}.
 * @param ctx the parse tree
 */
fn enter_callByContentPhrase(&mut self, _ctx: &CallByContentPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByContentPhrase}.
 * @param ctx the parse tree
 */
fn exit_callByContentPhrase(&mut self, _ctx: &CallByContentPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callByContent}.
 * @param ctx the parse tree
 */
fn enter_callByContent(&mut self, _ctx: &CallByContentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callByContent}.
 * @param ctx the parse tree
 */
fn exit_callByContent(&mut self, _ctx: &CallByContentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#callGivingPhrase}.
 * @param ctx the parse tree
 */
fn enter_callGivingPhrase(&mut self, _ctx: &CallGivingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#callGivingPhrase}.
 * @param ctx the parse tree
 */
fn exit_callGivingPhrase(&mut self, _ctx: &CallGivingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cancelStatement}.
 * @param ctx the parse tree
 */
fn enter_cancelStatement(&mut self, _ctx: &CancelStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cancelStatement}.
 * @param ctx the parse tree
 */
fn exit_cancelStatement(&mut self, _ctx: &CancelStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cancelCall}.
 * @param ctx the parse tree
 */
fn enter_cancelCall(&mut self, _ctx: &CancelCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cancelCall}.
 * @param ctx the parse tree
 */
fn exit_cancelCall(&mut self, _ctx: &CancelCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closeStatement}.
 * @param ctx the parse tree
 */
fn enter_closeStatement(&mut self, _ctx: &CloseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closeStatement}.
 * @param ctx the parse tree
 */
fn exit_closeStatement(&mut self, _ctx: &CloseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closeFile}.
 * @param ctx the parse tree
 */
fn enter_closeFile(&mut self, _ctx: &CloseFileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closeFile}.
 * @param ctx the parse tree
 */
fn exit_closeFile(&mut self, _ctx: &CloseFileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closeReelUnitStatement}.
 * @param ctx the parse tree
 */
fn enter_closeReelUnitStatement(&mut self, _ctx: &CloseReelUnitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closeReelUnitStatement}.
 * @param ctx the parse tree
 */
fn exit_closeReelUnitStatement(&mut self, _ctx: &CloseReelUnitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closeRelativeStatement}.
 * @param ctx the parse tree
 */
fn enter_closeRelativeStatement(&mut self, _ctx: &CloseRelativeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closeRelativeStatement}.
 * @param ctx the parse tree
 */
fn exit_closeRelativeStatement(&mut self, _ctx: &CloseRelativeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closePortFileIOStatement}.
 * @param ctx the parse tree
 */
fn enter_closePortFileIOStatement(&mut self, _ctx: &ClosePortFileIOStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closePortFileIOStatement}.
 * @param ctx the parse tree
 */
fn exit_closePortFileIOStatement(&mut self, _ctx: &ClosePortFileIOStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closePortFileIOUsing}.
 * @param ctx the parse tree
 */
fn enter_closePortFileIOUsing(&mut self, _ctx: &ClosePortFileIOUsingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsing}.
 * @param ctx the parse tree
 */
fn exit_closePortFileIOUsing(&mut self, _ctx: &ClosePortFileIOUsingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingCloseDisposition}.
 * @param ctx the parse tree
 */
fn enter_closePortFileIOUsingCloseDisposition(&mut self, _ctx: &ClosePortFileIOUsingCloseDispositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingCloseDisposition}.
 * @param ctx the parse tree
 */
fn exit_closePortFileIOUsingCloseDisposition(&mut self, _ctx: &ClosePortFileIOUsingCloseDispositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedData}.
 * @param ctx the parse tree
 */
fn enter_closePortFileIOUsingAssociatedData(&mut self, _ctx: &ClosePortFileIOUsingAssociatedDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedData}.
 * @param ctx the parse tree
 */
fn exit_closePortFileIOUsingAssociatedData(&mut self, _ctx: &ClosePortFileIOUsingAssociatedDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedDataLength}.
 * @param ctx the parse tree
 */
fn enter_closePortFileIOUsingAssociatedDataLength(&mut self, _ctx: &ClosePortFileIOUsingAssociatedDataLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#closePortFileIOUsingAssociatedDataLength}.
 * @param ctx the parse tree
 */
fn exit_closePortFileIOUsingAssociatedDataLength(&mut self, _ctx: &ClosePortFileIOUsingAssociatedDataLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#computeStatement}.
 * @param ctx the parse tree
 */
fn enter_computeStatement(&mut self, _ctx: &ComputeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#computeStatement}.
 * @param ctx the parse tree
 */
fn exit_computeStatement(&mut self, _ctx: &ComputeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#computeStore}.
 * @param ctx the parse tree
 */
fn enter_computeStore(&mut self, _ctx: &ComputeStoreContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#computeStore}.
 * @param ctx the parse tree
 */
fn exit_computeStore(&mut self, _ctx: &ComputeStoreContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#continueStatement}.
 * @param ctx the parse tree
 */
fn enter_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#continueStatement}.
 * @param ctx the parse tree
 */
fn exit_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#deleteStatement}.
 * @param ctx the parse tree
 */
fn enter_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#deleteStatement}.
 * @param ctx the parse tree
 */
fn exit_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#disableStatement}.
 * @param ctx the parse tree
 */
fn enter_disableStatement(&mut self, _ctx: &DisableStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#disableStatement}.
 * @param ctx the parse tree
 */
fn exit_disableStatement(&mut self, _ctx: &DisableStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#displayStatement}.
 * @param ctx the parse tree
 */
fn enter_displayStatement(&mut self, _ctx: &DisplayStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#displayStatement}.
 * @param ctx the parse tree
 */
fn exit_displayStatement(&mut self, _ctx: &DisplayStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#displayOperand}.
 * @param ctx the parse tree
 */
fn enter_displayOperand(&mut self, _ctx: &DisplayOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#displayOperand}.
 * @param ctx the parse tree
 */
fn exit_displayOperand(&mut self, _ctx: &DisplayOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#displayAt}.
 * @param ctx the parse tree
 */
fn enter_displayAt(&mut self, _ctx: &DisplayAtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#displayAt}.
 * @param ctx the parse tree
 */
fn exit_displayAt(&mut self, _ctx: &DisplayAtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#displayUpon}.
 * @param ctx the parse tree
 */
fn enter_displayUpon(&mut self, _ctx: &DisplayUponContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#displayUpon}.
 * @param ctx the parse tree
 */
fn exit_displayUpon(&mut self, _ctx: &DisplayUponContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#displayWith}.
 * @param ctx the parse tree
 */
fn enter_displayWith(&mut self, _ctx: &DisplayWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#displayWith}.
 * @param ctx the parse tree
 */
fn exit_displayWith(&mut self, _ctx: &DisplayWithContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideStatement}.
 * @param ctx the parse tree
 */
fn enter_divideStatement(&mut self, _ctx: &DivideStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideStatement}.
 * @param ctx the parse tree
 */
fn exit_divideStatement(&mut self, _ctx: &DivideStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideIntoStatement}.
 * @param ctx the parse tree
 */
fn enter_divideIntoStatement(&mut self, _ctx: &DivideIntoStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideIntoStatement}.
 * @param ctx the parse tree
 */
fn exit_divideIntoStatement(&mut self, _ctx: &DivideIntoStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideIntoGivingStatement}.
 * @param ctx the parse tree
 */
fn enter_divideIntoGivingStatement(&mut self, _ctx: &DivideIntoGivingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideIntoGivingStatement}.
 * @param ctx the parse tree
 */
fn exit_divideIntoGivingStatement(&mut self, _ctx: &DivideIntoGivingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideByGivingStatement}.
 * @param ctx the parse tree
 */
fn enter_divideByGivingStatement(&mut self, _ctx: &DivideByGivingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideByGivingStatement}.
 * @param ctx the parse tree
 */
fn exit_divideByGivingStatement(&mut self, _ctx: &DivideByGivingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideGivingPhrase}.
 * @param ctx the parse tree
 */
fn enter_divideGivingPhrase(&mut self, _ctx: &DivideGivingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideGivingPhrase}.
 * @param ctx the parse tree
 */
fn exit_divideGivingPhrase(&mut self, _ctx: &DivideGivingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideInto}.
 * @param ctx the parse tree
 */
fn enter_divideInto(&mut self, _ctx: &DivideIntoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideInto}.
 * @param ctx the parse tree
 */
fn exit_divideInto(&mut self, _ctx: &DivideIntoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideGiving}.
 * @param ctx the parse tree
 */
fn enter_divideGiving(&mut self, _ctx: &DivideGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideGiving}.
 * @param ctx the parse tree
 */
fn exit_divideGiving(&mut self, _ctx: &DivideGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#divideRemainder}.
 * @param ctx the parse tree
 */
fn enter_divideRemainder(&mut self, _ctx: &DivideRemainderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#divideRemainder}.
 * @param ctx the parse tree
 */
fn exit_divideRemainder(&mut self, _ctx: &DivideRemainderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#enableStatement}.
 * @param ctx the parse tree
 */
fn enter_enableStatement(&mut self, _ctx: &EnableStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#enableStatement}.
 * @param ctx the parse tree
 */
fn exit_enableStatement(&mut self, _ctx: &EnableStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#entryStatement}.
 * @param ctx the parse tree
 */
fn enter_entryStatement(&mut self, _ctx: &EntryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#entryStatement}.
 * @param ctx the parse tree
 */
fn exit_entryStatement(&mut self, _ctx: &EntryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateStatement}.
 * @param ctx the parse tree
 */
fn enter_evaluateStatement(&mut self, _ctx: &EvaluateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateStatement}.
 * @param ctx the parse tree
 */
fn exit_evaluateStatement(&mut self, _ctx: &EvaluateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateSelect}.
 * @param ctx the parse tree
 */
fn enter_evaluateSelect(&mut self, _ctx: &EvaluateSelectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateSelect}.
 * @param ctx the parse tree
 */
fn exit_evaluateSelect(&mut self, _ctx: &EvaluateSelectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateAlsoSelect}.
 * @param ctx the parse tree
 */
fn enter_evaluateAlsoSelect(&mut self, _ctx: &EvaluateAlsoSelectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateAlsoSelect}.
 * @param ctx the parse tree
 */
fn exit_evaluateAlsoSelect(&mut self, _ctx: &EvaluateAlsoSelectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateWhenPhrase}.
 * @param ctx the parse tree
 */
fn enter_evaluateWhenPhrase(&mut self, _ctx: &EvaluateWhenPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateWhenPhrase}.
 * @param ctx the parse tree
 */
fn exit_evaluateWhenPhrase(&mut self, _ctx: &EvaluateWhenPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateWhen}.
 * @param ctx the parse tree
 */
fn enter_evaluateWhen(&mut self, _ctx: &EvaluateWhenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateWhen}.
 * @param ctx the parse tree
 */
fn exit_evaluateWhen(&mut self, _ctx: &EvaluateWhenContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateCondition}.
 * @param ctx the parse tree
 */
fn enter_evaluateCondition(&mut self, _ctx: &EvaluateConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateCondition}.
 * @param ctx the parse tree
 */
fn exit_evaluateCondition(&mut self, _ctx: &EvaluateConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateThrough}.
 * @param ctx the parse tree
 */
fn enter_evaluateThrough(&mut self, _ctx: &EvaluateThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateThrough}.
 * @param ctx the parse tree
 */
fn exit_evaluateThrough(&mut self, _ctx: &EvaluateThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateAlsoCondition}.
 * @param ctx the parse tree
 */
fn enter_evaluateAlsoCondition(&mut self, _ctx: &EvaluateAlsoConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateAlsoCondition}.
 * @param ctx the parse tree
 */
fn exit_evaluateAlsoCondition(&mut self, _ctx: &EvaluateAlsoConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateWhenOther}.
 * @param ctx the parse tree
 */
fn enter_evaluateWhenOther(&mut self, _ctx: &EvaluateWhenOtherContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateWhenOther}.
 * @param ctx the parse tree
 */
fn exit_evaluateWhenOther(&mut self, _ctx: &EvaluateWhenOtherContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#evaluateValue}.
 * @param ctx the parse tree
 */
fn enter_evaluateValue(&mut self, _ctx: &EvaluateValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#evaluateValue}.
 * @param ctx the parse tree
 */
fn exit_evaluateValue(&mut self, _ctx: &EvaluateValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#execCicsStatement}.
 * @param ctx the parse tree
 */
fn enter_execCicsStatement(&mut self, _ctx: &ExecCicsStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#execCicsStatement}.
 * @param ctx the parse tree
 */
fn exit_execCicsStatement(&mut self, _ctx: &ExecCicsStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#execSqlStatement}.
 * @param ctx the parse tree
 */
fn enter_execSqlStatement(&mut self, _ctx: &ExecSqlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#execSqlStatement}.
 * @param ctx the parse tree
 */
fn exit_execSqlStatement(&mut self, _ctx: &ExecSqlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#execSqlImsStatement}.
 * @param ctx the parse tree
 */
fn enter_execSqlImsStatement(&mut self, _ctx: &ExecSqlImsStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#execSqlImsStatement}.
 * @param ctx the parse tree
 */
fn exit_execSqlImsStatement(&mut self, _ctx: &ExecSqlImsStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#exhibitStatement}.
 * @param ctx the parse tree
 */
fn enter_exhibitStatement(&mut self, _ctx: &ExhibitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#exhibitStatement}.
 * @param ctx the parse tree
 */
fn exit_exhibitStatement(&mut self, _ctx: &ExhibitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#exhibitOperand}.
 * @param ctx the parse tree
 */
fn enter_exhibitOperand(&mut self, _ctx: &ExhibitOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#exhibitOperand}.
 * @param ctx the parse tree
 */
fn exit_exhibitOperand(&mut self, _ctx: &ExhibitOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#exitStatement}.
 * @param ctx the parse tree
 */
fn enter_exitStatement(&mut self, _ctx: &ExitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#exitStatement}.
 * @param ctx the parse tree
 */
fn exit_exitStatement(&mut self, _ctx: &ExitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#generateStatement}.
 * @param ctx the parse tree
 */
fn enter_generateStatement(&mut self, _ctx: &GenerateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#generateStatement}.
 * @param ctx the parse tree
 */
fn exit_generateStatement(&mut self, _ctx: &GenerateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#gobackStatement}.
 * @param ctx the parse tree
 */
fn enter_gobackStatement(&mut self, _ctx: &GobackStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#gobackStatement}.
 * @param ctx the parse tree
 */
fn exit_gobackStatement(&mut self, _ctx: &GobackStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#goToStatement}.
 * @param ctx the parse tree
 */
fn enter_goToStatement(&mut self, _ctx: &GoToStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#goToStatement}.
 * @param ctx the parse tree
 */
fn exit_goToStatement(&mut self, _ctx: &GoToStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#goToStatementSimple}.
 * @param ctx the parse tree
 */
fn enter_goToStatementSimple(&mut self, _ctx: &GoToStatementSimpleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#goToStatementSimple}.
 * @param ctx the parse tree
 */
fn exit_goToStatementSimple(&mut self, _ctx: &GoToStatementSimpleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#goToDependingOnStatement}.
 * @param ctx the parse tree
 */
fn enter_goToDependingOnStatement(&mut self, _ctx: &GoToDependingOnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#goToDependingOnStatement}.
 * @param ctx the parse tree
 */
fn exit_goToDependingOnStatement(&mut self, _ctx: &GoToDependingOnStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#ifStatement}.
 * @param ctx the parse tree
 */
fn enter_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#ifStatement}.
 * @param ctx the parse tree
 */
fn exit_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#ifThen}.
 * @param ctx the parse tree
 */
fn enter_ifThen(&mut self, _ctx: &IfThenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#ifThen}.
 * @param ctx the parse tree
 */
fn exit_ifThen(&mut self, _ctx: &IfThenContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#ifElse}.
 * @param ctx the parse tree
 */
fn enter_ifElse(&mut self, _ctx: &IfElseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#ifElse}.
 * @param ctx the parse tree
 */
fn exit_ifElse(&mut self, _ctx: &IfElseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#initializeStatement}.
 * @param ctx the parse tree
 */
fn enter_initializeStatement(&mut self, _ctx: &InitializeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#initializeStatement}.
 * @param ctx the parse tree
 */
fn exit_initializeStatement(&mut self, _ctx: &InitializeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#initializeReplacingPhrase}.
 * @param ctx the parse tree
 */
fn enter_initializeReplacingPhrase(&mut self, _ctx: &InitializeReplacingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#initializeReplacingPhrase}.
 * @param ctx the parse tree
 */
fn exit_initializeReplacingPhrase(&mut self, _ctx: &InitializeReplacingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#initializeReplacingBy}.
 * @param ctx the parse tree
 */
fn enter_initializeReplacingBy(&mut self, _ctx: &InitializeReplacingByContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#initializeReplacingBy}.
 * @param ctx the parse tree
 */
fn exit_initializeReplacingBy(&mut self, _ctx: &InitializeReplacingByContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#initiateStatement}.
 * @param ctx the parse tree
 */
fn enter_initiateStatement(&mut self, _ctx: &InitiateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#initiateStatement}.
 * @param ctx the parse tree
 */
fn exit_initiateStatement(&mut self, _ctx: &InitiateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectStatement}.
 * @param ctx the parse tree
 */
fn enter_inspectStatement(&mut self, _ctx: &InspectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectStatement}.
 * @param ctx the parse tree
 */
fn exit_inspectStatement(&mut self, _ctx: &InspectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectTallyingPhrase}.
 * @param ctx the parse tree
 */
fn enter_inspectTallyingPhrase(&mut self, _ctx: &InspectTallyingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectTallyingPhrase}.
 * @param ctx the parse tree
 */
fn exit_inspectTallyingPhrase(&mut self, _ctx: &InspectTallyingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectReplacingPhrase}.
 * @param ctx the parse tree
 */
fn enter_inspectReplacingPhrase(&mut self, _ctx: &InspectReplacingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectReplacingPhrase}.
 * @param ctx the parse tree
 */
fn exit_inspectReplacingPhrase(&mut self, _ctx: &InspectReplacingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectTallyingReplacingPhrase}.
 * @param ctx the parse tree
 */
fn enter_inspectTallyingReplacingPhrase(&mut self, _ctx: &InspectTallyingReplacingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectTallyingReplacingPhrase}.
 * @param ctx the parse tree
 */
fn exit_inspectTallyingReplacingPhrase(&mut self, _ctx: &InspectTallyingReplacingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectConvertingPhrase}.
 * @param ctx the parse tree
 */
fn enter_inspectConvertingPhrase(&mut self, _ctx: &InspectConvertingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectConvertingPhrase}.
 * @param ctx the parse tree
 */
fn exit_inspectConvertingPhrase(&mut self, _ctx: &InspectConvertingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectFor}.
 * @param ctx the parse tree
 */
fn enter_inspectFor(&mut self, _ctx: &InspectForContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectFor}.
 * @param ctx the parse tree
 */
fn exit_inspectFor(&mut self, _ctx: &InspectForContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectCharacters}.
 * @param ctx the parse tree
 */
fn enter_inspectCharacters(&mut self, _ctx: &InspectCharactersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectCharacters}.
 * @param ctx the parse tree
 */
fn exit_inspectCharacters(&mut self, _ctx: &InspectCharactersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectReplacingCharacters}.
 * @param ctx the parse tree
 */
fn enter_inspectReplacingCharacters(&mut self, _ctx: &InspectReplacingCharactersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectReplacingCharacters}.
 * @param ctx the parse tree
 */
fn exit_inspectReplacingCharacters(&mut self, _ctx: &InspectReplacingCharactersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectAllLeadings}.
 * @param ctx the parse tree
 */
fn enter_inspectAllLeadings(&mut self, _ctx: &InspectAllLeadingsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectAllLeadings}.
 * @param ctx the parse tree
 */
fn exit_inspectAllLeadings(&mut self, _ctx: &InspectAllLeadingsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeadings}.
 * @param ctx the parse tree
 */
fn enter_inspectReplacingAllLeadings(&mut self, _ctx: &InspectReplacingAllLeadingsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeadings}.
 * @param ctx the parse tree
 */
fn exit_inspectReplacingAllLeadings(&mut self, _ctx: &InspectReplacingAllLeadingsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectAllLeading}.
 * @param ctx the parse tree
 */
fn enter_inspectAllLeading(&mut self, _ctx: &InspectAllLeadingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectAllLeading}.
 * @param ctx the parse tree
 */
fn exit_inspectAllLeading(&mut self, _ctx: &InspectAllLeadingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeading}.
 * @param ctx the parse tree
 */
fn enter_inspectReplacingAllLeading(&mut self, _ctx: &InspectReplacingAllLeadingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectReplacingAllLeading}.
 * @param ctx the parse tree
 */
fn exit_inspectReplacingAllLeading(&mut self, _ctx: &InspectReplacingAllLeadingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectBy}.
 * @param ctx the parse tree
 */
fn enter_inspectBy(&mut self, _ctx: &InspectByContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectBy}.
 * @param ctx the parse tree
 */
fn exit_inspectBy(&mut self, _ctx: &InspectByContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectTo}.
 * @param ctx the parse tree
 */
fn enter_inspectTo(&mut self, _ctx: &InspectToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectTo}.
 * @param ctx the parse tree
 */
fn exit_inspectTo(&mut self, _ctx: &InspectToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inspectBeforeAfter}.
 * @param ctx the parse tree
 */
fn enter_inspectBeforeAfter(&mut self, _ctx: &InspectBeforeAfterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inspectBeforeAfter}.
 * @param ctx the parse tree
 */
fn exit_inspectBeforeAfter(&mut self, _ctx: &InspectBeforeAfterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeStatement}.
 * @param ctx the parse tree
 */
fn enter_mergeStatement(&mut self, _ctx: &MergeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeStatement}.
 * @param ctx the parse tree
 */
fn exit_mergeStatement(&mut self, _ctx: &MergeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeOnKeyClause}.
 * @param ctx the parse tree
 */
fn enter_mergeOnKeyClause(&mut self, _ctx: &MergeOnKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeOnKeyClause}.
 * @param ctx the parse tree
 */
fn exit_mergeOnKeyClause(&mut self, _ctx: &MergeOnKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeCollatingSequencePhrase}.
 * @param ctx the parse tree
 */
fn enter_mergeCollatingSequencePhrase(&mut self, _ctx: &MergeCollatingSequencePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeCollatingSequencePhrase}.
 * @param ctx the parse tree
 */
fn exit_mergeCollatingSequencePhrase(&mut self, _ctx: &MergeCollatingSequencePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeCollatingAlphanumeric}.
 * @param ctx the parse tree
 */
fn enter_mergeCollatingAlphanumeric(&mut self, _ctx: &MergeCollatingAlphanumericContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeCollatingAlphanumeric}.
 * @param ctx the parse tree
 */
fn exit_mergeCollatingAlphanumeric(&mut self, _ctx: &MergeCollatingAlphanumericContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeCollatingNational}.
 * @param ctx the parse tree
 */
fn enter_mergeCollatingNational(&mut self, _ctx: &MergeCollatingNationalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeCollatingNational}.
 * @param ctx the parse tree
 */
fn exit_mergeCollatingNational(&mut self, _ctx: &MergeCollatingNationalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeUsing}.
 * @param ctx the parse tree
 */
fn enter_mergeUsing(&mut self, _ctx: &MergeUsingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeUsing}.
 * @param ctx the parse tree
 */
fn exit_mergeUsing(&mut self, _ctx: &MergeUsingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeOutputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn enter_mergeOutputProcedurePhrase(&mut self, _ctx: &MergeOutputProcedurePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeOutputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn exit_mergeOutputProcedurePhrase(&mut self, _ctx: &MergeOutputProcedurePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeOutputThrough}.
 * @param ctx the parse tree
 */
fn enter_mergeOutputThrough(&mut self, _ctx: &MergeOutputThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeOutputThrough}.
 * @param ctx the parse tree
 */
fn exit_mergeOutputThrough(&mut self, _ctx: &MergeOutputThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeGivingPhrase}.
 * @param ctx the parse tree
 */
fn enter_mergeGivingPhrase(&mut self, _ctx: &MergeGivingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeGivingPhrase}.
 * @param ctx the parse tree
 */
fn exit_mergeGivingPhrase(&mut self, _ctx: &MergeGivingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mergeGiving}.
 * @param ctx the parse tree
 */
fn enter_mergeGiving(&mut self, _ctx: &MergeGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mergeGiving}.
 * @param ctx the parse tree
 */
fn exit_mergeGiving(&mut self, _ctx: &MergeGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#moveStatement}.
 * @param ctx the parse tree
 */
fn enter_moveStatement(&mut self, _ctx: &MoveStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#moveStatement}.
 * @param ctx the parse tree
 */
fn exit_moveStatement(&mut self, _ctx: &MoveStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#moveToStatement}.
 * @param ctx the parse tree
 */
fn enter_moveToStatement(&mut self, _ctx: &MoveToStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#moveToStatement}.
 * @param ctx the parse tree
 */
fn exit_moveToStatement(&mut self, _ctx: &MoveToStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#moveToSendingArea}.
 * @param ctx the parse tree
 */
fn enter_moveToSendingArea(&mut self, _ctx: &MoveToSendingAreaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#moveToSendingArea}.
 * @param ctx the parse tree
 */
fn exit_moveToSendingArea(&mut self, _ctx: &MoveToSendingAreaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#moveCorrespondingToStatement}.
 * @param ctx the parse tree
 */
fn enter_moveCorrespondingToStatement(&mut self, _ctx: &MoveCorrespondingToStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToStatement}.
 * @param ctx the parse tree
 */
fn exit_moveCorrespondingToStatement(&mut self, _ctx: &MoveCorrespondingToStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#moveCorrespondingToSendingArea}.
 * @param ctx the parse tree
 */
fn enter_moveCorrespondingToSendingArea(&mut self, _ctx: &MoveCorrespondingToSendingAreaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#moveCorrespondingToSendingArea}.
 * @param ctx the parse tree
 */
fn exit_moveCorrespondingToSendingArea(&mut self, _ctx: &MoveCorrespondingToSendingAreaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyStatement}.
 * @param ctx the parse tree
 */
fn enter_multiplyStatement(&mut self, _ctx: &MultiplyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyStatement}.
 * @param ctx the parse tree
 */
fn exit_multiplyStatement(&mut self, _ctx: &MultiplyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyRegular}.
 * @param ctx the parse tree
 */
fn enter_multiplyRegular(&mut self, _ctx: &MultiplyRegularContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyRegular}.
 * @param ctx the parse tree
 */
fn exit_multiplyRegular(&mut self, _ctx: &MultiplyRegularContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyRegularOperand}.
 * @param ctx the parse tree
 */
fn enter_multiplyRegularOperand(&mut self, _ctx: &MultiplyRegularOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyRegularOperand}.
 * @param ctx the parse tree
 */
fn exit_multiplyRegularOperand(&mut self, _ctx: &MultiplyRegularOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyGiving}.
 * @param ctx the parse tree
 */
fn enter_multiplyGiving(&mut self, _ctx: &MultiplyGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyGiving}.
 * @param ctx the parse tree
 */
fn exit_multiplyGiving(&mut self, _ctx: &MultiplyGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyGivingOperand}.
 * @param ctx the parse tree
 */
fn enter_multiplyGivingOperand(&mut self, _ctx: &MultiplyGivingOperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyGivingOperand}.
 * @param ctx the parse tree
 */
fn exit_multiplyGivingOperand(&mut self, _ctx: &MultiplyGivingOperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multiplyGivingResult}.
 * @param ctx the parse tree
 */
fn enter_multiplyGivingResult(&mut self, _ctx: &MultiplyGivingResultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multiplyGivingResult}.
 * @param ctx the parse tree
 */
fn exit_multiplyGivingResult(&mut self, _ctx: &MultiplyGivingResultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openStatement}.
 * @param ctx the parse tree
 */
fn enter_openStatement(&mut self, _ctx: &OpenStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openStatement}.
 * @param ctx the parse tree
 */
fn exit_openStatement(&mut self, _ctx: &OpenStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openInputStatement}.
 * @param ctx the parse tree
 */
fn enter_openInputStatement(&mut self, _ctx: &OpenInputStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openInputStatement}.
 * @param ctx the parse tree
 */
fn exit_openInputStatement(&mut self, _ctx: &OpenInputStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openInput}.
 * @param ctx the parse tree
 */
fn enter_openInput(&mut self, _ctx: &OpenInputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openInput}.
 * @param ctx the parse tree
 */
fn exit_openInput(&mut self, _ctx: &OpenInputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openOutputStatement}.
 * @param ctx the parse tree
 */
fn enter_openOutputStatement(&mut self, _ctx: &OpenOutputStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openOutputStatement}.
 * @param ctx the parse tree
 */
fn exit_openOutputStatement(&mut self, _ctx: &OpenOutputStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openOutput}.
 * @param ctx the parse tree
 */
fn enter_openOutput(&mut self, _ctx: &OpenOutputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openOutput}.
 * @param ctx the parse tree
 */
fn exit_openOutput(&mut self, _ctx: &OpenOutputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openIOStatement}.
 * @param ctx the parse tree
 */
fn enter_openIOStatement(&mut self, _ctx: &OpenIOStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openIOStatement}.
 * @param ctx the parse tree
 */
fn exit_openIOStatement(&mut self, _ctx: &OpenIOStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#openExtendStatement}.
 * @param ctx the parse tree
 */
fn enter_openExtendStatement(&mut self, _ctx: &OpenExtendStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#openExtendStatement}.
 * @param ctx the parse tree
 */
fn exit_openExtendStatement(&mut self, _ctx: &OpenExtendStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performStatement}.
 * @param ctx the parse tree
 */
fn enter_performStatement(&mut self, _ctx: &PerformStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performStatement}.
 * @param ctx the parse tree
 */
fn exit_performStatement(&mut self, _ctx: &PerformStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performInlineStatement}.
 * @param ctx the parse tree
 */
fn enter_performInlineStatement(&mut self, _ctx: &PerformInlineStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performInlineStatement}.
 * @param ctx the parse tree
 */
fn exit_performInlineStatement(&mut self, _ctx: &PerformInlineStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performProcedureStatement}.
 * @param ctx the parse tree
 */
fn enter_performProcedureStatement(&mut self, _ctx: &PerformProcedureStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performProcedureStatement}.
 * @param ctx the parse tree
 */
fn exit_performProcedureStatement(&mut self, _ctx: &PerformProcedureStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performType}.
 * @param ctx the parse tree
 */
fn enter_performType(&mut self, _ctx: &PerformTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performType}.
 * @param ctx the parse tree
 */
fn exit_performType(&mut self, _ctx: &PerformTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performTimes}.
 * @param ctx the parse tree
 */
fn enter_performTimes(&mut self, _ctx: &PerformTimesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performTimes}.
 * @param ctx the parse tree
 */
fn exit_performTimes(&mut self, _ctx: &PerformTimesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performUntil}.
 * @param ctx the parse tree
 */
fn enter_performUntil(&mut self, _ctx: &PerformUntilContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performUntil}.
 * @param ctx the parse tree
 */
fn exit_performUntil(&mut self, _ctx: &PerformUntilContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performVarying}.
 * @param ctx the parse tree
 */
fn enter_performVarying(&mut self, _ctx: &PerformVaryingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performVarying}.
 * @param ctx the parse tree
 */
fn exit_performVarying(&mut self, _ctx: &PerformVaryingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performVaryingClause}.
 * @param ctx the parse tree
 */
fn enter_performVaryingClause(&mut self, _ctx: &PerformVaryingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performVaryingClause}.
 * @param ctx the parse tree
 */
fn exit_performVaryingClause(&mut self, _ctx: &PerformVaryingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performVaryingPhrase}.
 * @param ctx the parse tree
 */
fn enter_performVaryingPhrase(&mut self, _ctx: &PerformVaryingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performVaryingPhrase}.
 * @param ctx the parse tree
 */
fn exit_performVaryingPhrase(&mut self, _ctx: &PerformVaryingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performAfter}.
 * @param ctx the parse tree
 */
fn enter_performAfter(&mut self, _ctx: &PerformAfterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performAfter}.
 * @param ctx the parse tree
 */
fn exit_performAfter(&mut self, _ctx: &PerformAfterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performFrom}.
 * @param ctx the parse tree
 */
fn enter_performFrom(&mut self, _ctx: &PerformFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performFrom}.
 * @param ctx the parse tree
 */
fn exit_performFrom(&mut self, _ctx: &PerformFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performBy}.
 * @param ctx the parse tree
 */
fn enter_performBy(&mut self, _ctx: &PerformByContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performBy}.
 * @param ctx the parse tree
 */
fn exit_performBy(&mut self, _ctx: &PerformByContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#performTestClause}.
 * @param ctx the parse tree
 */
fn enter_performTestClause(&mut self, _ctx: &PerformTestClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#performTestClause}.
 * @param ctx the parse tree
 */
fn exit_performTestClause(&mut self, _ctx: &PerformTestClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#purgeStatement}.
 * @param ctx the parse tree
 */
fn enter_purgeStatement(&mut self, _ctx: &PurgeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#purgeStatement}.
 * @param ctx the parse tree
 */
fn exit_purgeStatement(&mut self, _ctx: &PurgeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#readStatement}.
 * @param ctx the parse tree
 */
fn enter_readStatement(&mut self, _ctx: &ReadStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#readStatement}.
 * @param ctx the parse tree
 */
fn exit_readStatement(&mut self, _ctx: &ReadStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#readInto}.
 * @param ctx the parse tree
 */
fn enter_readInto(&mut self, _ctx: &ReadIntoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#readInto}.
 * @param ctx the parse tree
 */
fn exit_readInto(&mut self, _ctx: &ReadIntoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#readWith}.
 * @param ctx the parse tree
 */
fn enter_readWith(&mut self, _ctx: &ReadWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#readWith}.
 * @param ctx the parse tree
 */
fn exit_readWith(&mut self, _ctx: &ReadWithContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#readKey}.
 * @param ctx the parse tree
 */
fn enter_readKey(&mut self, _ctx: &ReadKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#readKey}.
 * @param ctx the parse tree
 */
fn exit_readKey(&mut self, _ctx: &ReadKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveStatement}.
 * @param ctx the parse tree
 */
fn enter_receiveStatement(&mut self, _ctx: &ReceiveStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveStatement}.
 * @param ctx the parse tree
 */
fn exit_receiveStatement(&mut self, _ctx: &ReceiveStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveFromStatement}.
 * @param ctx the parse tree
 */
fn enter_receiveFromStatement(&mut self, _ctx: &ReceiveFromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveFromStatement}.
 * @param ctx the parse tree
 */
fn exit_receiveFromStatement(&mut self, _ctx: &ReceiveFromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveFrom}.
 * @param ctx the parse tree
 */
fn enter_receiveFrom(&mut self, _ctx: &ReceiveFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveFrom}.
 * @param ctx the parse tree
 */
fn exit_receiveFrom(&mut self, _ctx: &ReceiveFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveIntoStatement}.
 * @param ctx the parse tree
 */
fn enter_receiveIntoStatement(&mut self, _ctx: &ReceiveIntoStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveIntoStatement}.
 * @param ctx the parse tree
 */
fn exit_receiveIntoStatement(&mut self, _ctx: &ReceiveIntoStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveNoData}.
 * @param ctx the parse tree
 */
fn enter_receiveNoData(&mut self, _ctx: &ReceiveNoDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveNoData}.
 * @param ctx the parse tree
 */
fn exit_receiveNoData(&mut self, _ctx: &ReceiveNoDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveWithData}.
 * @param ctx the parse tree
 */
fn enter_receiveWithData(&mut self, _ctx: &ReceiveWithDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveWithData}.
 * @param ctx the parse tree
 */
fn exit_receiveWithData(&mut self, _ctx: &ReceiveWithDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveBefore}.
 * @param ctx the parse tree
 */
fn enter_receiveBefore(&mut self, _ctx: &ReceiveBeforeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveBefore}.
 * @param ctx the parse tree
 */
fn exit_receiveBefore(&mut self, _ctx: &ReceiveBeforeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveWith}.
 * @param ctx the parse tree
 */
fn enter_receiveWith(&mut self, _ctx: &ReceiveWithContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveWith}.
 * @param ctx the parse tree
 */
fn exit_receiveWith(&mut self, _ctx: &ReceiveWithContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveThread}.
 * @param ctx the parse tree
 */
fn enter_receiveThread(&mut self, _ctx: &ReceiveThreadContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveThread}.
 * @param ctx the parse tree
 */
fn exit_receiveThread(&mut self, _ctx: &ReceiveThreadContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveSize}.
 * @param ctx the parse tree
 */
fn enter_receiveSize(&mut self, _ctx: &ReceiveSizeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveSize}.
 * @param ctx the parse tree
 */
fn exit_receiveSize(&mut self, _ctx: &ReceiveSizeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#receiveStatus}.
 * @param ctx the parse tree
 */
fn enter_receiveStatus(&mut self, _ctx: &ReceiveStatusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#receiveStatus}.
 * @param ctx the parse tree
 */
fn exit_receiveStatus(&mut self, _ctx: &ReceiveStatusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#releaseStatement}.
 * @param ctx the parse tree
 */
fn enter_releaseStatement(&mut self, _ctx: &ReleaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#releaseStatement}.
 * @param ctx the parse tree
 */
fn exit_releaseStatement(&mut self, _ctx: &ReleaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#returnStatement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#returnStatement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#returnInto}.
 * @param ctx the parse tree
 */
fn enter_returnInto(&mut self, _ctx: &ReturnIntoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#returnInto}.
 * @param ctx the parse tree
 */
fn exit_returnInto(&mut self, _ctx: &ReturnIntoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rewriteStatement}.
 * @param ctx the parse tree
 */
fn enter_rewriteStatement(&mut self, _ctx: &RewriteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rewriteStatement}.
 * @param ctx the parse tree
 */
fn exit_rewriteStatement(&mut self, _ctx: &RewriteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#rewriteFrom}.
 * @param ctx the parse tree
 */
fn enter_rewriteFrom(&mut self, _ctx: &RewriteFromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#rewriteFrom}.
 * @param ctx the parse tree
 */
fn exit_rewriteFrom(&mut self, _ctx: &RewriteFromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#searchStatement}.
 * @param ctx the parse tree
 */
fn enter_searchStatement(&mut self, _ctx: &SearchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#searchStatement}.
 * @param ctx the parse tree
 */
fn exit_searchStatement(&mut self, _ctx: &SearchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#searchVarying}.
 * @param ctx the parse tree
 */
fn enter_searchVarying(&mut self, _ctx: &SearchVaryingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#searchVarying}.
 * @param ctx the parse tree
 */
fn exit_searchVarying(&mut self, _ctx: &SearchVaryingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#searchWhen}.
 * @param ctx the parse tree
 */
fn enter_searchWhen(&mut self, _ctx: &SearchWhenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#searchWhen}.
 * @param ctx the parse tree
 */
fn exit_searchWhen(&mut self, _ctx: &SearchWhenContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendStatement}.
 * @param ctx the parse tree
 */
fn enter_sendStatement(&mut self, _ctx: &SendStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendStatement}.
 * @param ctx the parse tree
 */
fn exit_sendStatement(&mut self, _ctx: &SendStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendStatementSync}.
 * @param ctx the parse tree
 */
fn enter_sendStatementSync(&mut self, _ctx: &SendStatementSyncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendStatementSync}.
 * @param ctx the parse tree
 */
fn exit_sendStatementSync(&mut self, _ctx: &SendStatementSyncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendStatementAsync}.
 * @param ctx the parse tree
 */
fn enter_sendStatementAsync(&mut self, _ctx: &SendStatementAsyncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendStatementAsync}.
 * @param ctx the parse tree
 */
fn exit_sendStatementAsync(&mut self, _ctx: &SendStatementAsyncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendFromPhrase}.
 * @param ctx the parse tree
 */
fn enter_sendFromPhrase(&mut self, _ctx: &SendFromPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendFromPhrase}.
 * @param ctx the parse tree
 */
fn exit_sendFromPhrase(&mut self, _ctx: &SendFromPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendWithPhrase}.
 * @param ctx the parse tree
 */
fn enter_sendWithPhrase(&mut self, _ctx: &SendWithPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendWithPhrase}.
 * @param ctx the parse tree
 */
fn exit_sendWithPhrase(&mut self, _ctx: &SendWithPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendReplacingPhrase}.
 * @param ctx the parse tree
 */
fn enter_sendReplacingPhrase(&mut self, _ctx: &SendReplacingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendReplacingPhrase}.
 * @param ctx the parse tree
 */
fn exit_sendReplacingPhrase(&mut self, _ctx: &SendReplacingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendAdvancingPhrase}.
 * @param ctx the parse tree
 */
fn enter_sendAdvancingPhrase(&mut self, _ctx: &SendAdvancingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendAdvancingPhrase}.
 * @param ctx the parse tree
 */
fn exit_sendAdvancingPhrase(&mut self, _ctx: &SendAdvancingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendAdvancingPage}.
 * @param ctx the parse tree
 */
fn enter_sendAdvancingPage(&mut self, _ctx: &SendAdvancingPageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendAdvancingPage}.
 * @param ctx the parse tree
 */
fn exit_sendAdvancingPage(&mut self, _ctx: &SendAdvancingPageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendAdvancingLines}.
 * @param ctx the parse tree
 */
fn enter_sendAdvancingLines(&mut self, _ctx: &SendAdvancingLinesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendAdvancingLines}.
 * @param ctx the parse tree
 */
fn exit_sendAdvancingLines(&mut self, _ctx: &SendAdvancingLinesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sendAdvancingMnemonic}.
 * @param ctx the parse tree
 */
fn enter_sendAdvancingMnemonic(&mut self, _ctx: &SendAdvancingMnemonicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sendAdvancingMnemonic}.
 * @param ctx the parse tree
 */
fn exit_sendAdvancingMnemonic(&mut self, _ctx: &SendAdvancingMnemonicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setStatement}.
 * @param ctx the parse tree
 */
fn enter_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setStatement}.
 * @param ctx the parse tree
 */
fn exit_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setToStatement}.
 * @param ctx the parse tree
 */
fn enter_setToStatement(&mut self, _ctx: &SetToStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setToStatement}.
 * @param ctx the parse tree
 */
fn exit_setToStatement(&mut self, _ctx: &SetToStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setUpDownByStatement}.
 * @param ctx the parse tree
 */
fn enter_setUpDownByStatement(&mut self, _ctx: &SetUpDownByStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setUpDownByStatement}.
 * @param ctx the parse tree
 */
fn exit_setUpDownByStatement(&mut self, _ctx: &SetUpDownByStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setTo}.
 * @param ctx the parse tree
 */
fn enter_setTo(&mut self, _ctx: &SetToContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setTo}.
 * @param ctx the parse tree
 */
fn exit_setTo(&mut self, _ctx: &SetToContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setToValue}.
 * @param ctx the parse tree
 */
fn enter_setToValue(&mut self, _ctx: &SetToValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setToValue}.
 * @param ctx the parse tree
 */
fn exit_setToValue(&mut self, _ctx: &SetToValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#setByValue}.
 * @param ctx the parse tree
 */
fn enter_setByValue(&mut self, _ctx: &SetByValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#setByValue}.
 * @param ctx the parse tree
 */
fn exit_setByValue(&mut self, _ctx: &SetByValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortStatement}.
 * @param ctx the parse tree
 */
fn enter_sortStatement(&mut self, _ctx: &SortStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortStatement}.
 * @param ctx the parse tree
 */
fn exit_sortStatement(&mut self, _ctx: &SortStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortOnKeyClause}.
 * @param ctx the parse tree
 */
fn enter_sortOnKeyClause(&mut self, _ctx: &SortOnKeyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortOnKeyClause}.
 * @param ctx the parse tree
 */
fn exit_sortOnKeyClause(&mut self, _ctx: &SortOnKeyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortDuplicatesPhrase}.
 * @param ctx the parse tree
 */
fn enter_sortDuplicatesPhrase(&mut self, _ctx: &SortDuplicatesPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortDuplicatesPhrase}.
 * @param ctx the parse tree
 */
fn exit_sortDuplicatesPhrase(&mut self, _ctx: &SortDuplicatesPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortCollatingSequencePhrase}.
 * @param ctx the parse tree
 */
fn enter_sortCollatingSequencePhrase(&mut self, _ctx: &SortCollatingSequencePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortCollatingSequencePhrase}.
 * @param ctx the parse tree
 */
fn exit_sortCollatingSequencePhrase(&mut self, _ctx: &SortCollatingSequencePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortCollatingAlphanumeric}.
 * @param ctx the parse tree
 */
fn enter_sortCollatingAlphanumeric(&mut self, _ctx: &SortCollatingAlphanumericContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortCollatingAlphanumeric}.
 * @param ctx the parse tree
 */
fn exit_sortCollatingAlphanumeric(&mut self, _ctx: &SortCollatingAlphanumericContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortCollatingNational}.
 * @param ctx the parse tree
 */
fn enter_sortCollatingNational(&mut self, _ctx: &SortCollatingNationalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortCollatingNational}.
 * @param ctx the parse tree
 */
fn exit_sortCollatingNational(&mut self, _ctx: &SortCollatingNationalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortInputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn enter_sortInputProcedurePhrase(&mut self, _ctx: &SortInputProcedurePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortInputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn exit_sortInputProcedurePhrase(&mut self, _ctx: &SortInputProcedurePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortInputThrough}.
 * @param ctx the parse tree
 */
fn enter_sortInputThrough(&mut self, _ctx: &SortInputThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortInputThrough}.
 * @param ctx the parse tree
 */
fn exit_sortInputThrough(&mut self, _ctx: &SortInputThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortUsing}.
 * @param ctx the parse tree
 */
fn enter_sortUsing(&mut self, _ctx: &SortUsingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortUsing}.
 * @param ctx the parse tree
 */
fn exit_sortUsing(&mut self, _ctx: &SortUsingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortOutputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn enter_sortOutputProcedurePhrase(&mut self, _ctx: &SortOutputProcedurePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortOutputProcedurePhrase}.
 * @param ctx the parse tree
 */
fn exit_sortOutputProcedurePhrase(&mut self, _ctx: &SortOutputProcedurePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortOutputThrough}.
 * @param ctx the parse tree
 */
fn enter_sortOutputThrough(&mut self, _ctx: &SortOutputThroughContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortOutputThrough}.
 * @param ctx the parse tree
 */
fn exit_sortOutputThrough(&mut self, _ctx: &SortOutputThroughContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortGivingPhrase}.
 * @param ctx the parse tree
 */
fn enter_sortGivingPhrase(&mut self, _ctx: &SortGivingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortGivingPhrase}.
 * @param ctx the parse tree
 */
fn exit_sortGivingPhrase(&mut self, _ctx: &SortGivingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sortGiving}.
 * @param ctx the parse tree
 */
fn enter_sortGiving(&mut self, _ctx: &SortGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sortGiving}.
 * @param ctx the parse tree
 */
fn exit_sortGiving(&mut self, _ctx: &SortGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#startStatement}.
 * @param ctx the parse tree
 */
fn enter_startStatement(&mut self, _ctx: &StartStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#startStatement}.
 * @param ctx the parse tree
 */
fn exit_startStatement(&mut self, _ctx: &StartStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#startKey}.
 * @param ctx the parse tree
 */
fn enter_startKey(&mut self, _ctx: &StartKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#startKey}.
 * @param ctx the parse tree
 */
fn exit_startKey(&mut self, _ctx: &StartKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stopStatement}.
 * @param ctx the parse tree
 */
fn enter_stopStatement(&mut self, _ctx: &StopStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stopStatement}.
 * @param ctx the parse tree
 */
fn exit_stopStatement(&mut self, _ctx: &StopStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringStatement}.
 * @param ctx the parse tree
 */
fn enter_stringStatement(&mut self, _ctx: &StringStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringStatement}.
 * @param ctx the parse tree
 */
fn exit_stringStatement(&mut self, _ctx: &StringStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringSendingPhrase}.
 * @param ctx the parse tree
 */
fn enter_stringSendingPhrase(&mut self, _ctx: &StringSendingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringSendingPhrase}.
 * @param ctx the parse tree
 */
fn exit_stringSendingPhrase(&mut self, _ctx: &StringSendingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringSending}.
 * @param ctx the parse tree
 */
fn enter_stringSending(&mut self, _ctx: &StringSendingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringSending}.
 * @param ctx the parse tree
 */
fn exit_stringSending(&mut self, _ctx: &StringSendingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringDelimitedByPhrase}.
 * @param ctx the parse tree
 */
fn enter_stringDelimitedByPhrase(&mut self, _ctx: &StringDelimitedByPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringDelimitedByPhrase}.
 * @param ctx the parse tree
 */
fn exit_stringDelimitedByPhrase(&mut self, _ctx: &StringDelimitedByPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringForPhrase}.
 * @param ctx the parse tree
 */
fn enter_stringForPhrase(&mut self, _ctx: &StringForPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringForPhrase}.
 * @param ctx the parse tree
 */
fn exit_stringForPhrase(&mut self, _ctx: &StringForPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringIntoPhrase}.
 * @param ctx the parse tree
 */
fn enter_stringIntoPhrase(&mut self, _ctx: &StringIntoPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringIntoPhrase}.
 * @param ctx the parse tree
 */
fn exit_stringIntoPhrase(&mut self, _ctx: &StringIntoPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#stringWithPointerPhrase}.
 * @param ctx the parse tree
 */
fn enter_stringWithPointerPhrase(&mut self, _ctx: &StringWithPointerPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#stringWithPointerPhrase}.
 * @param ctx the parse tree
 */
fn exit_stringWithPointerPhrase(&mut self, _ctx: &StringWithPointerPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractStatement}.
 * @param ctx the parse tree
 */
fn enter_subtractStatement(&mut self, _ctx: &SubtractStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractStatement}.
 * @param ctx the parse tree
 */
fn exit_subtractStatement(&mut self, _ctx: &SubtractStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractFromStatement}.
 * @param ctx the parse tree
 */
fn enter_subtractFromStatement(&mut self, _ctx: &SubtractFromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractFromStatement}.
 * @param ctx the parse tree
 */
fn exit_subtractFromStatement(&mut self, _ctx: &SubtractFromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractFromGivingStatement}.
 * @param ctx the parse tree
 */
fn enter_subtractFromGivingStatement(&mut self, _ctx: &SubtractFromGivingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractFromGivingStatement}.
 * @param ctx the parse tree
 */
fn exit_subtractFromGivingStatement(&mut self, _ctx: &SubtractFromGivingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractCorrespondingStatement}.
 * @param ctx the parse tree
 */
fn enter_subtractCorrespondingStatement(&mut self, _ctx: &SubtractCorrespondingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractCorrespondingStatement}.
 * @param ctx the parse tree
 */
fn exit_subtractCorrespondingStatement(&mut self, _ctx: &SubtractCorrespondingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractSubtrahend}.
 * @param ctx the parse tree
 */
fn enter_subtractSubtrahend(&mut self, _ctx: &SubtractSubtrahendContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractSubtrahend}.
 * @param ctx the parse tree
 */
fn exit_subtractSubtrahend(&mut self, _ctx: &SubtractSubtrahendContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractMinuend}.
 * @param ctx the parse tree
 */
fn enter_subtractMinuend(&mut self, _ctx: &SubtractMinuendContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractMinuend}.
 * @param ctx the parse tree
 */
fn exit_subtractMinuend(&mut self, _ctx: &SubtractMinuendContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractMinuendGiving}.
 * @param ctx the parse tree
 */
fn enter_subtractMinuendGiving(&mut self, _ctx: &SubtractMinuendGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractMinuendGiving}.
 * @param ctx the parse tree
 */
fn exit_subtractMinuendGiving(&mut self, _ctx: &SubtractMinuendGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractGiving}.
 * @param ctx the parse tree
 */
fn enter_subtractGiving(&mut self, _ctx: &SubtractGivingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractGiving}.
 * @param ctx the parse tree
 */
fn exit_subtractGiving(&mut self, _ctx: &SubtractGivingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subtractMinuendCorresponding}.
 * @param ctx the parse tree
 */
fn enter_subtractMinuendCorresponding(&mut self, _ctx: &SubtractMinuendCorrespondingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subtractMinuendCorresponding}.
 * @param ctx the parse tree
 */
fn exit_subtractMinuendCorresponding(&mut self, _ctx: &SubtractMinuendCorrespondingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#terminateStatement}.
 * @param ctx the parse tree
 */
fn enter_terminateStatement(&mut self, _ctx: &TerminateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#terminateStatement}.
 * @param ctx the parse tree
 */
fn exit_terminateStatement(&mut self, _ctx: &TerminateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringStatement}.
 * @param ctx the parse tree
 */
fn enter_unstringStatement(&mut self, _ctx: &UnstringStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringStatement}.
 * @param ctx the parse tree
 */
fn exit_unstringStatement(&mut self, _ctx: &UnstringStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringSendingPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringSendingPhrase(&mut self, _ctx: &UnstringSendingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringSendingPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringSendingPhrase(&mut self, _ctx: &UnstringSendingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringDelimitedByPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringDelimitedByPhrase(&mut self, _ctx: &UnstringDelimitedByPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringDelimitedByPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringDelimitedByPhrase(&mut self, _ctx: &UnstringDelimitedByPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringOrAllPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringOrAllPhrase(&mut self, _ctx: &UnstringOrAllPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringOrAllPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringOrAllPhrase(&mut self, _ctx: &UnstringOrAllPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringIntoPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringIntoPhrase(&mut self, _ctx: &UnstringIntoPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringIntoPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringIntoPhrase(&mut self, _ctx: &UnstringIntoPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringInto}.
 * @param ctx the parse tree
 */
fn enter_unstringInto(&mut self, _ctx: &UnstringIntoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringInto}.
 * @param ctx the parse tree
 */
fn exit_unstringInto(&mut self, _ctx: &UnstringIntoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringDelimiterIn}.
 * @param ctx the parse tree
 */
fn enter_unstringDelimiterIn(&mut self, _ctx: &UnstringDelimiterInContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringDelimiterIn}.
 * @param ctx the parse tree
 */
fn exit_unstringDelimiterIn(&mut self, _ctx: &UnstringDelimiterInContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringCountIn}.
 * @param ctx the parse tree
 */
fn enter_unstringCountIn(&mut self, _ctx: &UnstringCountInContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringCountIn}.
 * @param ctx the parse tree
 */
fn exit_unstringCountIn(&mut self, _ctx: &UnstringCountInContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringWithPointerPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringWithPointerPhrase(&mut self, _ctx: &UnstringWithPointerPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringWithPointerPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringWithPointerPhrase(&mut self, _ctx: &UnstringWithPointerPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#unstringTallyingPhrase}.
 * @param ctx the parse tree
 */
fn enter_unstringTallyingPhrase(&mut self, _ctx: &UnstringTallyingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#unstringTallyingPhrase}.
 * @param ctx the parse tree
 */
fn exit_unstringTallyingPhrase(&mut self, _ctx: &UnstringTallyingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#useStatement}.
 * @param ctx the parse tree
 */
fn enter_useStatement(&mut self, _ctx: &UseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#useStatement}.
 * @param ctx the parse tree
 */
fn exit_useStatement(&mut self, _ctx: &UseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#useAfterClause}.
 * @param ctx the parse tree
 */
fn enter_useAfterClause(&mut self, _ctx: &UseAfterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#useAfterClause}.
 * @param ctx the parse tree
 */
fn exit_useAfterClause(&mut self, _ctx: &UseAfterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#useAfterOn}.
 * @param ctx the parse tree
 */
fn enter_useAfterOn(&mut self, _ctx: &UseAfterOnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#useAfterOn}.
 * @param ctx the parse tree
 */
fn exit_useAfterOn(&mut self, _ctx: &UseAfterOnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#useDebugClause}.
 * @param ctx the parse tree
 */
fn enter_useDebugClause(&mut self, _ctx: &UseDebugClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#useDebugClause}.
 * @param ctx the parse tree
 */
fn exit_useDebugClause(&mut self, _ctx: &UseDebugClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#useDebugOn}.
 * @param ctx the parse tree
 */
fn enter_useDebugOn(&mut self, _ctx: &UseDebugOnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#useDebugOn}.
 * @param ctx the parse tree
 */
fn exit_useDebugOn(&mut self, _ctx: &UseDebugOnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeStatement}.
 * @param ctx the parse tree
 */
fn enter_writeStatement(&mut self, _ctx: &WriteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeStatement}.
 * @param ctx the parse tree
 */
fn exit_writeStatement(&mut self, _ctx: &WriteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeFromPhrase}.
 * @param ctx the parse tree
 */
fn enter_writeFromPhrase(&mut self, _ctx: &WriteFromPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeFromPhrase}.
 * @param ctx the parse tree
 */
fn exit_writeFromPhrase(&mut self, _ctx: &WriteFromPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeAdvancingPhrase}.
 * @param ctx the parse tree
 */
fn enter_writeAdvancingPhrase(&mut self, _ctx: &WriteAdvancingPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeAdvancingPhrase}.
 * @param ctx the parse tree
 */
fn exit_writeAdvancingPhrase(&mut self, _ctx: &WriteAdvancingPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeAdvancingPage}.
 * @param ctx the parse tree
 */
fn enter_writeAdvancingPage(&mut self, _ctx: &WriteAdvancingPageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeAdvancingPage}.
 * @param ctx the parse tree
 */
fn exit_writeAdvancingPage(&mut self, _ctx: &WriteAdvancingPageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeAdvancingLines}.
 * @param ctx the parse tree
 */
fn enter_writeAdvancingLines(&mut self, _ctx: &WriteAdvancingLinesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeAdvancingLines}.
 * @param ctx the parse tree
 */
fn exit_writeAdvancingLines(&mut self, _ctx: &WriteAdvancingLinesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeAdvancingMnemonic}.
 * @param ctx the parse tree
 */
fn enter_writeAdvancingMnemonic(&mut self, _ctx: &WriteAdvancingMnemonicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeAdvancingMnemonic}.
 * @param ctx the parse tree
 */
fn exit_writeAdvancingMnemonic(&mut self, _ctx: &WriteAdvancingMnemonicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeAtEndOfPagePhrase}.
 * @param ctx the parse tree
 */
fn enter_writeAtEndOfPagePhrase(&mut self, _ctx: &WriteAtEndOfPagePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeAtEndOfPagePhrase}.
 * @param ctx the parse tree
 */
fn exit_writeAtEndOfPagePhrase(&mut self, _ctx: &WriteAtEndOfPagePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#writeNotAtEndOfPagePhrase}.
 * @param ctx the parse tree
 */
fn enter_writeNotAtEndOfPagePhrase(&mut self, _ctx: &WriteNotAtEndOfPagePhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#writeNotAtEndOfPagePhrase}.
 * @param ctx the parse tree
 */
fn exit_writeNotAtEndOfPagePhrase(&mut self, _ctx: &WriteNotAtEndOfPagePhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#atEndPhrase}.
 * @param ctx the parse tree
 */
fn enter_atEndPhrase(&mut self, _ctx: &AtEndPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#atEndPhrase}.
 * @param ctx the parse tree
 */
fn exit_atEndPhrase(&mut self, _ctx: &AtEndPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#notAtEndPhrase}.
 * @param ctx the parse tree
 */
fn enter_notAtEndPhrase(&mut self, _ctx: &NotAtEndPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#notAtEndPhrase}.
 * @param ctx the parse tree
 */
fn exit_notAtEndPhrase(&mut self, _ctx: &NotAtEndPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#invalidKeyPhrase}.
 * @param ctx the parse tree
 */
fn enter_invalidKeyPhrase(&mut self, _ctx: &InvalidKeyPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#invalidKeyPhrase}.
 * @param ctx the parse tree
 */
fn exit_invalidKeyPhrase(&mut self, _ctx: &InvalidKeyPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#notInvalidKeyPhrase}.
 * @param ctx the parse tree
 */
fn enter_notInvalidKeyPhrase(&mut self, _ctx: &NotInvalidKeyPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#notInvalidKeyPhrase}.
 * @param ctx the parse tree
 */
fn exit_notInvalidKeyPhrase(&mut self, _ctx: &NotInvalidKeyPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#onOverflowPhrase}.
 * @param ctx the parse tree
 */
fn enter_onOverflowPhrase(&mut self, _ctx: &OnOverflowPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#onOverflowPhrase}.
 * @param ctx the parse tree
 */
fn exit_onOverflowPhrase(&mut self, _ctx: &OnOverflowPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#notOnOverflowPhrase}.
 * @param ctx the parse tree
 */
fn enter_notOnOverflowPhrase(&mut self, _ctx: &NotOnOverflowPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#notOnOverflowPhrase}.
 * @param ctx the parse tree
 */
fn exit_notOnOverflowPhrase(&mut self, _ctx: &NotOnOverflowPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#onSizeErrorPhrase}.
 * @param ctx the parse tree
 */
fn enter_onSizeErrorPhrase(&mut self, _ctx: &OnSizeErrorPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#onSizeErrorPhrase}.
 * @param ctx the parse tree
 */
fn exit_onSizeErrorPhrase(&mut self, _ctx: &OnSizeErrorPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#notOnSizeErrorPhrase}.
 * @param ctx the parse tree
 */
fn enter_notOnSizeErrorPhrase(&mut self, _ctx: &NotOnSizeErrorPhraseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#notOnSizeErrorPhrase}.
 * @param ctx the parse tree
 */
fn exit_notOnSizeErrorPhrase(&mut self, _ctx: &NotOnSizeErrorPhraseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#onExceptionClause}.
 * @param ctx the parse tree
 */
fn enter_onExceptionClause(&mut self, _ctx: &OnExceptionClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#onExceptionClause}.
 * @param ctx the parse tree
 */
fn exit_onExceptionClause(&mut self, _ctx: &OnExceptionClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#notOnExceptionClause}.
 * @param ctx the parse tree
 */
fn enter_notOnExceptionClause(&mut self, _ctx: &NotOnExceptionClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#notOnExceptionClause}.
 * @param ctx the parse tree
 */
fn exit_notOnExceptionClause(&mut self, _ctx: &NotOnExceptionClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#arithmeticExpression}.
 * @param ctx the parse tree
 */
fn enter_arithmeticExpression(&mut self, _ctx: &ArithmeticExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#arithmeticExpression}.
 * @param ctx the parse tree
 */
fn exit_arithmeticExpression(&mut self, _ctx: &ArithmeticExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#plusMinus}.
 * @param ctx the parse tree
 */
fn enter_plusMinus(&mut self, _ctx: &PlusMinusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#plusMinus}.
 * @param ctx the parse tree
 */
fn exit_plusMinus(&mut self, _ctx: &PlusMinusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multDivs}.
 * @param ctx the parse tree
 */
fn enter_multDivs(&mut self, _ctx: &MultDivsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multDivs}.
 * @param ctx the parse tree
 */
fn exit_multDivs(&mut self, _ctx: &MultDivsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#multDiv}.
 * @param ctx the parse tree
 */
fn enter_multDiv(&mut self, _ctx: &MultDivContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#multDiv}.
 * @param ctx the parse tree
 */
fn exit_multDiv(&mut self, _ctx: &MultDivContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#powers}.
 * @param ctx the parse tree
 */
fn enter_powers(&mut self, _ctx: &PowersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#powers}.
 * @param ctx the parse tree
 */
fn exit_powers(&mut self, _ctx: &PowersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#power}.
 * @param ctx the parse tree
 */
fn enter_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#power}.
 * @param ctx the parse tree
 */
fn exit_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#basis}.
 * @param ctx the parse tree
 */
fn enter_basis(&mut self, _ctx: &BasisContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#basis}.
 * @param ctx the parse tree
 */
fn exit_basis(&mut self, _ctx: &BasisContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#condition}.
 * @param ctx the parse tree
 */
fn enter_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#condition}.
 * @param ctx the parse tree
 */
fn exit_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#andOrCondition}.
 * @param ctx the parse tree
 */
fn enter_andOrCondition(&mut self, _ctx: &AndOrConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#andOrCondition}.
 * @param ctx the parse tree
 */
fn exit_andOrCondition(&mut self, _ctx: &AndOrConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#combinableCondition}.
 * @param ctx the parse tree
 */
fn enter_combinableCondition(&mut self, _ctx: &CombinableConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#combinableCondition}.
 * @param ctx the parse tree
 */
fn exit_combinableCondition(&mut self, _ctx: &CombinableConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#simpleCondition}.
 * @param ctx the parse tree
 */
fn enter_simpleCondition(&mut self, _ctx: &SimpleConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#simpleCondition}.
 * @param ctx the parse tree
 */
fn exit_simpleCondition(&mut self, _ctx: &SimpleConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#classCondition}.
 * @param ctx the parse tree
 */
fn enter_classCondition(&mut self, _ctx: &ClassConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#classCondition}.
 * @param ctx the parse tree
 */
fn exit_classCondition(&mut self, _ctx: &ClassConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#conditionNameReference}.
 * @param ctx the parse tree
 */
fn enter_conditionNameReference(&mut self, _ctx: &ConditionNameReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#conditionNameReference}.
 * @param ctx the parse tree
 */
fn exit_conditionNameReference(&mut self, _ctx: &ConditionNameReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#conditionNameSubscriptReference}.
 * @param ctx the parse tree
 */
fn enter_conditionNameSubscriptReference(&mut self, _ctx: &ConditionNameSubscriptReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#conditionNameSubscriptReference}.
 * @param ctx the parse tree
 */
fn exit_conditionNameSubscriptReference(&mut self, _ctx: &ConditionNameSubscriptReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationCondition}.
 * @param ctx the parse tree
 */
fn enter_relationCondition(&mut self, _ctx: &RelationConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationCondition}.
 * @param ctx the parse tree
 */
fn exit_relationCondition(&mut self, _ctx: &RelationConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationSignCondition}.
 * @param ctx the parse tree
 */
fn enter_relationSignCondition(&mut self, _ctx: &RelationSignConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationSignCondition}.
 * @param ctx the parse tree
 */
fn exit_relationSignCondition(&mut self, _ctx: &RelationSignConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationArithmeticComparison}.
 * @param ctx the parse tree
 */
fn enter_relationArithmeticComparison(&mut self, _ctx: &RelationArithmeticComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationArithmeticComparison}.
 * @param ctx the parse tree
 */
fn exit_relationArithmeticComparison(&mut self, _ctx: &RelationArithmeticComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationCombinedComparison}.
 * @param ctx the parse tree
 */
fn enter_relationCombinedComparison(&mut self, _ctx: &RelationCombinedComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationCombinedComparison}.
 * @param ctx the parse tree
 */
fn exit_relationCombinedComparison(&mut self, _ctx: &RelationCombinedComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationCombinedCondition}.
 * @param ctx the parse tree
 */
fn enter_relationCombinedCondition(&mut self, _ctx: &RelationCombinedConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationCombinedCondition}.
 * @param ctx the parse tree
 */
fn exit_relationCombinedCondition(&mut self, _ctx: &RelationCombinedConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#relationalOperator}.
 * @param ctx the parse tree
 */
fn enter_relationalOperator(&mut self, _ctx: &RelationalOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#relationalOperator}.
 * @param ctx the parse tree
 */
fn exit_relationalOperator(&mut self, _ctx: &RelationalOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#abbreviation}.
 * @param ctx the parse tree
 */
fn enter_abbreviation(&mut self, _ctx: &AbbreviationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#abbreviation}.
 * @param ctx the parse tree
 */
fn exit_abbreviation(&mut self, _ctx: &AbbreviationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#tableCall}.
 * @param ctx the parse tree
 */
fn enter_tableCall(&mut self, _ctx: &TableCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#tableCall}.
 * @param ctx the parse tree
 */
fn exit_tableCall(&mut self, _ctx: &TableCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#referenceModifier}.
 * @param ctx the parse tree
 */
fn enter_referenceModifier(&mut self, _ctx: &ReferenceModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#referenceModifier}.
 * @param ctx the parse tree
 */
fn exit_referenceModifier(&mut self, _ctx: &ReferenceModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#characterPosition}.
 * @param ctx the parse tree
 */
fn enter_characterPosition(&mut self, _ctx: &CharacterPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#characterPosition}.
 * @param ctx the parse tree
 */
fn exit_characterPosition(&mut self, _ctx: &CharacterPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#length}.
 * @param ctx the parse tree
 */
fn enter_length(&mut self, _ctx: &LengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#length}.
 * @param ctx the parse tree
 */
fn exit_length(&mut self, _ctx: &LengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#subscript_}.
 * @param ctx the parse tree
 */
fn enter_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#subscript_}.
 * @param ctx the parse tree
 */
fn exit_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#argument}.
 * @param ctx the parse tree
 */
fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#argument}.
 * @param ctx the parse tree
 */
fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedDataName}.
 * @param ctx the parse tree
 */
fn enter_qualifiedDataName(&mut self, _ctx: &QualifiedDataNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedDataName}.
 * @param ctx the parse tree
 */
fn exit_qualifiedDataName(&mut self, _ctx: &QualifiedDataNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat1}.
 * @param ctx the parse tree
 */
fn enter_qualifiedDataNameFormat1(&mut self, _ctx: &QualifiedDataNameFormat1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat1}.
 * @param ctx the parse tree
 */
fn exit_qualifiedDataNameFormat1(&mut self, _ctx: &QualifiedDataNameFormat1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat2}.
 * @param ctx the parse tree
 */
fn enter_qualifiedDataNameFormat2(&mut self, _ctx: &QualifiedDataNameFormat2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat2}.
 * @param ctx the parse tree
 */
fn exit_qualifiedDataNameFormat2(&mut self, _ctx: &QualifiedDataNameFormat2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat3}.
 * @param ctx the parse tree
 */
fn enter_qualifiedDataNameFormat3(&mut self, _ctx: &QualifiedDataNameFormat3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat3}.
 * @param ctx the parse tree
 */
fn exit_qualifiedDataNameFormat3(&mut self, _ctx: &QualifiedDataNameFormat3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat4}.
 * @param ctx the parse tree
 */
fn enter_qualifiedDataNameFormat4(&mut self, _ctx: &QualifiedDataNameFormat4Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedDataNameFormat4}.
 * @param ctx the parse tree
 */
fn exit_qualifiedDataNameFormat4(&mut self, _ctx: &QualifiedDataNameFormat4Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#qualifiedInData}.
 * @param ctx the parse tree
 */
fn enter_qualifiedInData(&mut self, _ctx: &QualifiedInDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#qualifiedInData}.
 * @param ctx the parse tree
 */
fn exit_qualifiedInData(&mut self, _ctx: &QualifiedInDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inData}.
 * @param ctx the parse tree
 */
fn enter_inData(&mut self, _ctx: &InDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inData}.
 * @param ctx the parse tree
 */
fn exit_inData(&mut self, _ctx: &InDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inFile}.
 * @param ctx the parse tree
 */
fn enter_inFile(&mut self, _ctx: &InFileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inFile}.
 * @param ctx the parse tree
 */
fn exit_inFile(&mut self, _ctx: &InFileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inMnemonic}.
 * @param ctx the parse tree
 */
fn enter_inMnemonic(&mut self, _ctx: &InMnemonicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inMnemonic}.
 * @param ctx the parse tree
 */
fn exit_inMnemonic(&mut self, _ctx: &InMnemonicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inSection}.
 * @param ctx the parse tree
 */
fn enter_inSection(&mut self, _ctx: &InSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inSection}.
 * @param ctx the parse tree
 */
fn exit_inSection(&mut self, _ctx: &InSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inLibrary}.
 * @param ctx the parse tree
 */
fn enter_inLibrary(&mut self, _ctx: &InLibraryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inLibrary}.
 * @param ctx the parse tree
 */
fn exit_inLibrary(&mut self, _ctx: &InLibraryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#inTable}.
 * @param ctx the parse tree
 */
fn enter_inTable(&mut self, _ctx: &InTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#inTable}.
 * @param ctx the parse tree
 */
fn exit_inTable(&mut self, _ctx: &InTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#alphabetName}.
 * @param ctx the parse tree
 */
fn enter_alphabetName(&mut self, _ctx: &AlphabetNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#alphabetName}.
 * @param ctx the parse tree
 */
fn exit_alphabetName(&mut self, _ctx: &AlphabetNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#assignmentName}.
 * @param ctx the parse tree
 */
fn enter_assignmentName(&mut self, _ctx: &AssignmentNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#assignmentName}.
 * @param ctx the parse tree
 */
fn exit_assignmentName(&mut self, _ctx: &AssignmentNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#basisName}.
 * @param ctx the parse tree
 */
fn enter_basisName(&mut self, _ctx: &BasisNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#basisName}.
 * @param ctx the parse tree
 */
fn exit_basisName(&mut self, _ctx: &BasisNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cdName}.
 * @param ctx the parse tree
 */
fn enter_cdName(&mut self, _ctx: &CdNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cdName}.
 * @param ctx the parse tree
 */
fn exit_cdName(&mut self, _ctx: &CdNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#className}.
 * @param ctx the parse tree
 */
fn enter_className(&mut self, _ctx: &ClassNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#className}.
 * @param ctx the parse tree
 */
fn exit_className(&mut self, _ctx: &ClassNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#computerName}.
 * @param ctx the parse tree
 */
fn enter_computerName(&mut self, _ctx: &ComputerNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#computerName}.
 * @param ctx the parse tree
 */
fn exit_computerName(&mut self, _ctx: &ComputerNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#conditionName}.
 * @param ctx the parse tree
 */
fn enter_conditionName(&mut self, _ctx: &ConditionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#conditionName}.
 * @param ctx the parse tree
 */
fn exit_conditionName(&mut self, _ctx: &ConditionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataName}.
 * @param ctx the parse tree
 */
fn enter_dataName(&mut self, _ctx: &DataNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataName}.
 * @param ctx the parse tree
 */
fn exit_dataName(&mut self, _ctx: &DataNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#dataDescName}.
 * @param ctx the parse tree
 */
fn enter_dataDescName(&mut self, _ctx: &DataDescNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#dataDescName}.
 * @param ctx the parse tree
 */
fn exit_dataDescName(&mut self, _ctx: &DataDescNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#environmentName}.
 * @param ctx the parse tree
 */
fn enter_environmentName(&mut self, _ctx: &EnvironmentNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#environmentName}.
 * @param ctx the parse tree
 */
fn exit_environmentName(&mut self, _ctx: &EnvironmentNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#fileName}.
 * @param ctx the parse tree
 */
fn enter_fileName(&mut self, _ctx: &FileNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#fileName}.
 * @param ctx the parse tree
 */
fn exit_fileName(&mut self, _ctx: &FileNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#indexName}.
 * @param ctx the parse tree
 */
fn enter_indexName(&mut self, _ctx: &IndexNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#indexName}.
 * @param ctx the parse tree
 */
fn exit_indexName(&mut self, _ctx: &IndexNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#languageName}.
 * @param ctx the parse tree
 */
fn enter_languageName(&mut self, _ctx: &LanguageNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#languageName}.
 * @param ctx the parse tree
 */
fn exit_languageName(&mut self, _ctx: &LanguageNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#libraryName}.
 * @param ctx the parse tree
 */
fn enter_libraryName(&mut self, _ctx: &LibraryNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#libraryName}.
 * @param ctx the parse tree
 */
fn exit_libraryName(&mut self, _ctx: &LibraryNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#localName}.
 * @param ctx the parse tree
 */
fn enter_localName(&mut self, _ctx: &LocalNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#localName}.
 * @param ctx the parse tree
 */
fn exit_localName(&mut self, _ctx: &LocalNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#mnemonicName}.
 * @param ctx the parse tree
 */
fn enter_mnemonicName(&mut self, _ctx: &MnemonicNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#mnemonicName}.
 * @param ctx the parse tree
 */
fn exit_mnemonicName(&mut self, _ctx: &MnemonicNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#paragraphName}.
 * @param ctx the parse tree
 */
fn enter_paragraphName(&mut self, _ctx: &ParagraphNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#paragraphName}.
 * @param ctx the parse tree
 */
fn exit_paragraphName(&mut self, _ctx: &ParagraphNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#procedureName}.
 * @param ctx the parse tree
 */
fn enter_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#procedureName}.
 * @param ctx the parse tree
 */
fn exit_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#programName}.
 * @param ctx the parse tree
 */
fn enter_programName(&mut self, _ctx: &ProgramNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#programName}.
 * @param ctx the parse tree
 */
fn exit_programName(&mut self, _ctx: &ProgramNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#recordName}.
 * @param ctx the parse tree
 */
fn enter_recordName(&mut self, _ctx: &RecordNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#recordName}.
 * @param ctx the parse tree
 */
fn exit_recordName(&mut self, _ctx: &RecordNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#reportName}.
 * @param ctx the parse tree
 */
fn enter_reportName(&mut self, _ctx: &ReportNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#reportName}.
 * @param ctx the parse tree
 */
fn exit_reportName(&mut self, _ctx: &ReportNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#routineName}.
 * @param ctx the parse tree
 */
fn enter_routineName(&mut self, _ctx: &RoutineNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#routineName}.
 * @param ctx the parse tree
 */
fn exit_routineName(&mut self, _ctx: &RoutineNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#screenName}.
 * @param ctx the parse tree
 */
fn enter_screenName(&mut self, _ctx: &ScreenNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#screenName}.
 * @param ctx the parse tree
 */
fn exit_screenName(&mut self, _ctx: &ScreenNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#sectionName}.
 * @param ctx the parse tree
 */
fn enter_sectionName(&mut self, _ctx: &SectionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#sectionName}.
 * @param ctx the parse tree
 */
fn exit_sectionName(&mut self, _ctx: &SectionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#systemName}.
 * @param ctx the parse tree
 */
fn enter_systemName(&mut self, _ctx: &SystemNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#systemName}.
 * @param ctx the parse tree
 */
fn exit_systemName(&mut self, _ctx: &SystemNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#symbolicCharacter}.
 * @param ctx the parse tree
 */
fn enter_symbolicCharacter(&mut self, _ctx: &SymbolicCharacterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#symbolicCharacter}.
 * @param ctx the parse tree
 */
fn exit_symbolicCharacter(&mut self, _ctx: &SymbolicCharacterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#textName}.
 * @param ctx the parse tree
 */
fn enter_textName(&mut self, _ctx: &TextNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#textName}.
 * @param ctx the parse tree
 */
fn exit_textName(&mut self, _ctx: &TextNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cobolWord}.
 * @param ctx the parse tree
 */
fn enter_cobolWord(&mut self, _ctx: &CobolWordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cobolWord}.
 * @param ctx the parse tree
 */
fn exit_cobolWord(&mut self, _ctx: &CobolWordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#numericLiteral}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#numericLiteral}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#integerLiteral}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#integerLiteral}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cicsDfhRespLiteral}.
 * @param ctx the parse tree
 */
fn enter_cicsDfhRespLiteral(&mut self, _ctx: &CicsDfhRespLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cicsDfhRespLiteral}.
 * @param ctx the parse tree
 */
fn exit_cicsDfhRespLiteral(&mut self, _ctx: &CicsDfhRespLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#cicsDfhValueLiteral}.
 * @param ctx the parse tree
 */
fn enter_cicsDfhValueLiteral(&mut self, _ctx: &CicsDfhValueLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#cicsDfhValueLiteral}.
 * @param ctx the parse tree
 */
fn exit_cicsDfhValueLiteral(&mut self, _ctx: &CicsDfhValueLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#figurativeConstant}.
 * @param ctx the parse tree
 */
fn enter_figurativeConstant(&mut self, _ctx: &FigurativeConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#figurativeConstant}.
 * @param ctx the parse tree
 */
fn exit_figurativeConstant(&mut self, _ctx: &FigurativeConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#specialRegister}.
 * @param ctx the parse tree
 */
fn enter_specialRegister(&mut self, _ctx: &SpecialRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#specialRegister}.
 * @param ctx the parse tree
 */
fn exit_specialRegister(&mut self, _ctx: &SpecialRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Cobol85Parser#commentEntry}.
 * @param ctx the parse tree
 */
fn enter_commentEntry(&mut self, _ctx: &CommentEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Cobol85Parser#commentEntry}.
 * @param ctx the parse tree
 */
fn exit_commentEntry(&mut self, _ctx: &CommentEntryContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : Cobol85Listener<'input> }


