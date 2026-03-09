// Generated from /Users/chandramohn/workspace/cobol2rust/grammar/Cobol85Preprocessor.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::cobol85preprocessorlistener::*;
use super::cobol85preprocessorvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const ADATA:isize=1; 
		pub const ADV:isize=2; 
		pub const ALIAS:isize=3; 
		pub const ANSI:isize=4; 
		pub const ANY:isize=5; 
		pub const APOST:isize=6; 
		pub const AR:isize=7; 
		pub const ARITH:isize=8; 
		pub const AUTO:isize=9; 
		pub const AWO:isize=10; 
		pub const BIN:isize=11; 
		pub const BLOCK0:isize=12; 
		pub const BUF:isize=13; 
		pub const BUFSIZE:isize=14; 
		pub const BY:isize=15; 
		pub const CBL:isize=16; 
		pub const CBLCARD:isize=17; 
		pub const CICS:isize=18; 
		pub const CO:isize=19; 
		pub const COBOL2:isize=20; 
		pub const COBOL3:isize=21; 
		pub const CODEPAGE:isize=22; 
		pub const COMPAT:isize=23; 
		pub const COMPILE:isize=24; 
		pub const COPY:isize=25; 
		pub const CP:isize=26; 
		pub const CPP:isize=27; 
		pub const CPSM:isize=28; 
		pub const CS:isize=29; 
		pub const CURR:isize=30; 
		pub const CURRENCY:isize=31; 
		pub const DATA:isize=32; 
		pub const DATEPROC:isize=33; 
		pub const DBCS:isize=34; 
		pub const DD:isize=35; 
		pub const DEBUG:isize=36; 
		pub const DECK:isize=37; 
		pub const DIAGTRUNC:isize=38; 
		pub const DLI:isize=39; 
		pub const DLL:isize=40; 
		pub const DP:isize=41; 
		pub const DTR:isize=42; 
		pub const DU:isize=43; 
		pub const DUMP:isize=44; 
		pub const DYN:isize=45; 
		pub const DYNAM:isize=46; 
		pub const EDF:isize=47; 
		pub const EJECT:isize=48; 
		pub const EJPD:isize=49; 
		pub const EN:isize=50; 
		pub const ENGLISH:isize=51; 
		pub const END_EXEC:isize=52; 
		pub const EPILOG:isize=53; 
		pub const EXCI:isize=54; 
		pub const EXEC:isize=55; 
		pub const EXIT:isize=56; 
		pub const EXP:isize=57; 
		pub const EXPORTALL:isize=58; 
		pub const EXTEND:isize=59; 
		pub const FASTSRT:isize=60; 
		pub const FEPI:isize=61; 
		pub const FLAG:isize=62; 
		pub const FLAGSTD:isize=63; 
		pub const FSRT:isize=64; 
		pub const FULL:isize=65; 
		pub const GDS:isize=66; 
		pub const GRAPHIC:isize=67; 
		pub const HOOK:isize=68; 
		pub const IN:isize=69; 
		pub const INTDATE:isize=70; 
		pub const JA:isize=71; 
		pub const JP:isize=72; 
		pub const KA:isize=73; 
		pub const LANG:isize=74; 
		pub const LANGUAGE:isize=75; 
		pub const LC:isize=76; 
		pub const LEASM:isize=77; 
		pub const LENGTH:isize=78; 
		pub const LIB:isize=79; 
		pub const LILIAN:isize=80; 
		pub const LIN:isize=81; 
		pub const LINECOUNT:isize=82; 
		pub const LINKAGE:isize=83; 
		pub const LIST:isize=84; 
		pub const LM:isize=85; 
		pub const LONGMIXED:isize=86; 
		pub const LONGUPPER:isize=87; 
		pub const LPARENCHAR:isize=88; 
		pub const LU:isize=89; 
		pub const MAP:isize=90; 
		pub const MARGINS:isize=91; 
		pub const MAX:isize=92; 
		pub const MD:isize=93; 
		pub const MDECK:isize=94; 
		pub const MIG:isize=95; 
		pub const MIXED:isize=96; 
		pub const NAME:isize=97; 
		pub const NAT:isize=98; 
		pub const NATIONAL:isize=99; 
		pub const NATLANG:isize=100; 
		pub const NN:isize=101; 
		pub const NO:isize=102; 
		pub const NOADATA:isize=103; 
		pub const NOADV:isize=104; 
		pub const NOALIAS:isize=105; 
		pub const NOAWO:isize=106; 
		pub const NOBLOCK0:isize=107; 
		pub const NOC:isize=108; 
		pub const NOCBLCARD:isize=109; 
		pub const NOCICS:isize=110; 
		pub const NOCMPR2:isize=111; 
		pub const NOCOMPILE:isize=112; 
		pub const NOCPSM:isize=113; 
		pub const NOCURR:isize=114; 
		pub const NOCURRENCY:isize=115; 
		pub const NOD:isize=116; 
		pub const NODATEPROC:isize=117; 
		pub const NODBCS:isize=118; 
		pub const NODE:isize=119; 
		pub const NODEBUG:isize=120; 
		pub const NODECK:isize=121; 
		pub const NODIAGTRUNC:isize=122; 
		pub const NODLL:isize=123; 
		pub const NODU:isize=124; 
		pub const NODUMP:isize=125; 
		pub const NODP:isize=126; 
		pub const NODTR:isize=127; 
		pub const NODYN:isize=128; 
		pub const NODYNAM:isize=129; 
		pub const NOEDF:isize=130; 
		pub const NOEJPD:isize=131; 
		pub const NOEPILOG:isize=132; 
		pub const NOEXIT:isize=133; 
		pub const NOEXP:isize=134; 
		pub const NOEXPORTALL:isize=135; 
		pub const NOF:isize=136; 
		pub const NOFASTSRT:isize=137; 
		pub const NOFEPI:isize=138; 
		pub const NOFLAG:isize=139; 
		pub const NOFLAGMIG:isize=140; 
		pub const NOFLAGSTD:isize=141; 
		pub const NOFSRT:isize=142; 
		pub const NOGRAPHIC:isize=143; 
		pub const NOHOOK:isize=144; 
		pub const NOLENGTH:isize=145; 
		pub const NOLIB:isize=146; 
		pub const NOLINKAGE:isize=147; 
		pub const NOLIST:isize=148; 
		pub const NOMAP:isize=149; 
		pub const NOMD:isize=150; 
		pub const NOMDECK:isize=151; 
		pub const NONAME:isize=152; 
		pub const NONUM:isize=153; 
		pub const NONUMBER:isize=154; 
		pub const NOOBJ:isize=155; 
		pub const NOOBJECT:isize=156; 
		pub const NOOFF:isize=157; 
		pub const NOOFFSET:isize=158; 
		pub const NOOPSEQUENCE:isize=159; 
		pub const NOOPT:isize=160; 
		pub const NOOPTIMIZE:isize=161; 
		pub const NOOPTIONS:isize=162; 
		pub const NOP:isize=163; 
		pub const NOPFD:isize=164; 
		pub const NOPROLOG:isize=165; 
		pub const NORENT:isize=166; 
		pub const NOS:isize=167; 
		pub const NOSEP:isize=168; 
		pub const NOSEPARATE:isize=169; 
		pub const NOSEQ:isize=170; 
		pub const NOSOURCE:isize=171; 
		pub const NOSPIE:isize=172; 
		pub const NOSQL:isize=173; 
		pub const NOSQLC:isize=174; 
		pub const NOSQLCCSID:isize=175; 
		pub const NOSSR:isize=176; 
		pub const NOSSRANGE:isize=177; 
		pub const NOSTDTRUNC:isize=178; 
		pub const NOSEQUENCE:isize=179; 
		pub const NOTERM:isize=180; 
		pub const NOTERMINAL:isize=181; 
		pub const NOTEST:isize=182; 
		pub const NOTHREAD:isize=183; 
		pub const NOTRIG:isize=184; 
		pub const NOVBREF:isize=185; 
		pub const NOWD:isize=186; 
		pub const NOWORD:isize=187; 
		pub const NOX:isize=188; 
		pub const NOXREF:isize=189; 
		pub const NOZWB:isize=190; 
		pub const NS:isize=191; 
		pub const NSEQ:isize=192; 
		pub const NSYMBOL:isize=193; 
		pub const NUM:isize=194; 
		pub const NUMBER:isize=195; 
		pub const NUMPROC:isize=196; 
		pub const OBJ:isize=197; 
		pub const OBJECT:isize=198; 
		pub const OF:isize=199; 
		pub const OFF:isize=200; 
		pub const OFFSET:isize=201; 
		pub const ON:isize=202; 
		pub const OP:isize=203; 
		pub const OPMARGINS:isize=204; 
		pub const OPSEQUENCE:isize=205; 
		pub const OPT:isize=206; 
		pub const OPTFILE:isize=207; 
		pub const OPTIMIZE:isize=208; 
		pub const OPTIONS:isize=209; 
		pub const OUT:isize=210; 
		pub const OUTDD:isize=211; 
		pub const PFD:isize=212; 
		pub const PPTDBG:isize=213; 
		pub const PGMN:isize=214; 
		pub const PGMNAME:isize=215; 
		pub const PROCESS:isize=216; 
		pub const PROLOG:isize=217; 
		pub const QUOTE:isize=218; 
		pub const RENT:isize=219; 
		pub const REPLACE:isize=220; 
		pub const REPLACING:isize=221; 
		pub const RMODE:isize=222; 
		pub const RPARENCHAR:isize=223; 
		pub const SEP:isize=224; 
		pub const SEPARATE:isize=225; 
		pub const SEQ:isize=226; 
		pub const SEQUENCE:isize=227; 
		pub const SHORT:isize=228; 
		pub const SIZE:isize=229; 
		pub const SOURCE:isize=230; 
		pub const SP:isize=231; 
		pub const SPACE:isize=232; 
		pub const SPIE:isize=233; 
		pub const SQL:isize=234; 
		pub const SQLC:isize=235; 
		pub const SQLCCSID:isize=236; 
		pub const SQLIMS:isize=237; 
		pub const SKIP1:isize=238; 
		pub const SKIP2:isize=239; 
		pub const SKIP3:isize=240; 
		pub const SS:isize=241; 
		pub const SSR:isize=242; 
		pub const SSRANGE:isize=243; 
		pub const STD:isize=244; 
		pub const SUPPRESS:isize=245; 
		pub const SYSEIB:isize=246; 
		pub const SZ:isize=247; 
		pub const TERM:isize=248; 
		pub const TERMINAL:isize=249; 
		pub const TEST:isize=250; 
		pub const THREAD:isize=251; 
		pub const TITLE:isize=252; 
		pub const TRIG:isize=253; 
		pub const TRUNC:isize=254; 
		pub const UE:isize=255; 
		pub const UPPER:isize=256; 
		pub const VBREF:isize=257; 
		pub const WD:isize=258; 
		pub const WORD:isize=259; 
		pub const XMLPARSE:isize=260; 
		pub const XMLSS:isize=261; 
		pub const XOPTS:isize=262; 
		pub const XP:isize=263; 
		pub const XREF:isize=264; 
		pub const YEARWINDOW:isize=265; 
		pub const YW:isize=266; 
		pub const ZWB:isize=267; 
		pub const C_CHAR:isize=268; 
		pub const D_CHAR:isize=269; 
		pub const E_CHAR:isize=270; 
		pub const F_CHAR:isize=271; 
		pub const H_CHAR:isize=272; 
		pub const I_CHAR:isize=273; 
		pub const M_CHAR:isize=274; 
		pub const N_CHAR:isize=275; 
		pub const Q_CHAR:isize=276; 
		pub const S_CHAR:isize=277; 
		pub const U_CHAR:isize=278; 
		pub const W_CHAR:isize=279; 
		pub const X_CHAR:isize=280; 
		pub const COMMENTTAG:isize=281; 
		pub const COMMACHAR:isize=282; 
		pub const DOT:isize=283; 
		pub const DOUBLEEQUALCHAR:isize=284; 
		pub const NONNUMERICLITERAL:isize=285; 
		pub const NUMERICLITERAL:isize=286; 
		pub const IDENTIFIER:isize=287; 
		pub const FILENAME:isize=288; 
		pub const NEWLINE:isize=289; 
		pub const COMMENTLINE:isize=290; 
		pub const WS:isize=291; 
		pub const TEXT:isize=292;
	pub const RULE_startRule:usize = 0; 
	pub const RULE_compilerOptions:usize = 1; 
	pub const RULE_compilerXOpts:usize = 2; 
	pub const RULE_compilerOption:usize = 3; 
	pub const RULE_execCicsStatement:usize = 4; 
	pub const RULE_execSqlStatement:usize = 5; 
	pub const RULE_execSqlImsStatement:usize = 6; 
	pub const RULE_copyStatement:usize = 7; 
	pub const RULE_copySource:usize = 8; 
	pub const RULE_copyLibrary:usize = 9; 
	pub const RULE_replacingPhrase:usize = 10; 
	pub const RULE_replaceArea:usize = 11; 
	pub const RULE_replaceByStatement:usize = 12; 
	pub const RULE_replaceOffStatement:usize = 13; 
	pub const RULE_replaceClause:usize = 14; 
	pub const RULE_directoryPhrase:usize = 15; 
	pub const RULE_familyPhrase:usize = 16; 
	pub const RULE_replaceable:usize = 17; 
	pub const RULE_replacement:usize = 18; 
	pub const RULE_ejectStatement:usize = 19; 
	pub const RULE_skipStatement:usize = 20; 
	pub const RULE_titleStatement:usize = 21; 
	pub const RULE_pseudoText:usize = 22; 
	pub const RULE_charData:usize = 23; 
	pub const RULE_charDataSql:usize = 24; 
	pub const RULE_charDataLine:usize = 25; 
	pub const RULE_cobolWord:usize = 26; 
	pub const RULE_literal:usize = 27; 
	pub const RULE_filename:usize = 28; 
	pub const RULE_charDataKeyword:usize = 29;
	pub const ruleNames: [&'static str; 30] =  [
		"startRule", "compilerOptions", "compilerXOpts", "compilerOption", "execCicsStatement", 
		"execSqlStatement", "execSqlImsStatement", "copyStatement", "copySource", 
		"copyLibrary", "replacingPhrase", "replaceArea", "replaceByStatement", 
		"replaceOffStatement", "replaceClause", "directoryPhrase", "familyPhrase", 
		"replaceable", "replacement", "ejectStatement", "skipStatement", "titleStatement", 
		"pseudoText", "charData", "charDataSql", "charDataLine", "cobolWord", 
		"literal", "filename", "charDataKeyword"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;285] = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, Some("'('"), None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, Some("')'"), None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, Some("'*>'"), Some("','"), Some("'.'"), 
		Some("'=='")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;293]  = [
		None, Some("ADATA"), Some("ADV"), Some("ALIAS"), Some("ANSI"), Some("ANY"), 
		Some("APOST"), Some("AR"), Some("ARITH"), Some("AUTO"), Some("AWO"), Some("BIN"), 
		Some("BLOCK0"), Some("BUF"), Some("BUFSIZE"), Some("BY"), Some("CBL"), 
		Some("CBLCARD"), Some("CICS"), Some("CO"), Some("COBOL2"), Some("COBOL3"), 
		Some("CODEPAGE"), Some("COMPAT"), Some("COMPILE"), Some("COPY"), Some("CP"), 
		Some("CPP"), Some("CPSM"), Some("CS"), Some("CURR"), Some("CURRENCY"), 
		Some("DATA"), Some("DATEPROC"), Some("DBCS"), Some("DD"), Some("DEBUG"), 
		Some("DECK"), Some("DIAGTRUNC"), Some("DLI"), Some("DLL"), Some("DP"), 
		Some("DTR"), Some("DU"), Some("DUMP"), Some("DYN"), Some("DYNAM"), Some("EDF"), 
		Some("EJECT"), Some("EJPD"), Some("EN"), Some("ENGLISH"), Some("END_EXEC"), 
		Some("EPILOG"), Some("EXCI"), Some("EXEC"), Some("EXIT"), Some("EXP"), 
		Some("EXPORTALL"), Some("EXTEND"), Some("FASTSRT"), Some("FEPI"), Some("FLAG"), 
		Some("FLAGSTD"), Some("FSRT"), Some("FULL"), Some("GDS"), Some("GRAPHIC"), 
		Some("HOOK"), Some("IN"), Some("INTDATE"), Some("JA"), Some("JP"), Some("KA"), 
		Some("LANG"), Some("LANGUAGE"), Some("LC"), Some("LEASM"), Some("LENGTH"), 
		Some("LIB"), Some("LILIAN"), Some("LIN"), Some("LINECOUNT"), Some("LINKAGE"), 
		Some("LIST"), Some("LM"), Some("LONGMIXED"), Some("LONGUPPER"), Some("LPARENCHAR"), 
		Some("LU"), Some("MAP"), Some("MARGINS"), Some("MAX"), Some("MD"), Some("MDECK"), 
		Some("MIG"), Some("MIXED"), Some("NAME"), Some("NAT"), Some("NATIONAL"), 
		Some("NATLANG"), Some("NN"), Some("NO"), Some("NOADATA"), Some("NOADV"), 
		Some("NOALIAS"), Some("NOAWO"), Some("NOBLOCK0"), Some("NOC"), Some("NOCBLCARD"), 
		Some("NOCICS"), Some("NOCMPR2"), Some("NOCOMPILE"), Some("NOCPSM"), Some("NOCURR"), 
		Some("NOCURRENCY"), Some("NOD"), Some("NODATEPROC"), Some("NODBCS"), Some("NODE"), 
		Some("NODEBUG"), Some("NODECK"), Some("NODIAGTRUNC"), Some("NODLL"), Some("NODU"), 
		Some("NODUMP"), Some("NODP"), Some("NODTR"), Some("NODYN"), Some("NODYNAM"), 
		Some("NOEDF"), Some("NOEJPD"), Some("NOEPILOG"), Some("NOEXIT"), Some("NOEXP"), 
		Some("NOEXPORTALL"), Some("NOF"), Some("NOFASTSRT"), Some("NOFEPI"), Some("NOFLAG"), 
		Some("NOFLAGMIG"), Some("NOFLAGSTD"), Some("NOFSRT"), Some("NOGRAPHIC"), 
		Some("NOHOOK"), Some("NOLENGTH"), Some("NOLIB"), Some("NOLINKAGE"), Some("NOLIST"), 
		Some("NOMAP"), Some("NOMD"), Some("NOMDECK"), Some("NONAME"), Some("NONUM"), 
		Some("NONUMBER"), Some("NOOBJ"), Some("NOOBJECT"), Some("NOOFF"), Some("NOOFFSET"), 
		Some("NOOPSEQUENCE"), Some("NOOPT"), Some("NOOPTIMIZE"), Some("NOOPTIONS"), 
		Some("NOP"), Some("NOPFD"), Some("NOPROLOG"), Some("NORENT"), Some("NOS"), 
		Some("NOSEP"), Some("NOSEPARATE"), Some("NOSEQ"), Some("NOSOURCE"), Some("NOSPIE"), 
		Some("NOSQL"), Some("NOSQLC"), Some("NOSQLCCSID"), Some("NOSSR"), Some("NOSSRANGE"), 
		Some("NOSTDTRUNC"), Some("NOSEQUENCE"), Some("NOTERM"), Some("NOTERMINAL"), 
		Some("NOTEST"), Some("NOTHREAD"), Some("NOTRIG"), Some("NOVBREF"), Some("NOWD"), 
		Some("NOWORD"), Some("NOX"), Some("NOXREF"), Some("NOZWB"), Some("NS"), 
		Some("NSEQ"), Some("NSYMBOL"), Some("NUM"), Some("NUMBER"), Some("NUMPROC"), 
		Some("OBJ"), Some("OBJECT"), Some("OF"), Some("OFF"), Some("OFFSET"), 
		Some("ON"), Some("OP"), Some("OPMARGINS"), Some("OPSEQUENCE"), Some("OPT"), 
		Some("OPTFILE"), Some("OPTIMIZE"), Some("OPTIONS"), Some("OUT"), Some("OUTDD"), 
		Some("PFD"), Some("PPTDBG"), Some("PGMN"), Some("PGMNAME"), Some("PROCESS"), 
		Some("PROLOG"), Some("QUOTE"), Some("RENT"), Some("REPLACE"), Some("REPLACING"), 
		Some("RMODE"), Some("RPARENCHAR"), Some("SEP"), Some("SEPARATE"), Some("SEQ"), 
		Some("SEQUENCE"), Some("SHORT"), Some("SIZE"), Some("SOURCE"), Some("SP"), 
		Some("SPACE"), Some("SPIE"), Some("SQL"), Some("SQLC"), Some("SQLCCSID"), 
		Some("SQLIMS"), Some("SKIP1"), Some("SKIP2"), Some("SKIP3"), Some("SS"), 
		Some("SSR"), Some("SSRANGE"), Some("STD"), Some("SUPPRESS"), Some("SYSEIB"), 
		Some("SZ"), Some("TERM"), Some("TERMINAL"), Some("TEST"), Some("THREAD"), 
		Some("TITLE"), Some("TRIG"), Some("TRUNC"), Some("UE"), Some("UPPER"), 
		Some("VBREF"), Some("WD"), Some("WORD"), Some("XMLPARSE"), Some("XMLSS"), 
		Some("XOPTS"), Some("XP"), Some("XREF"), Some("YEARWINDOW"), Some("YW"), 
		Some("ZWB"), Some("C_CHAR"), Some("D_CHAR"), Some("E_CHAR"), Some("F_CHAR"), 
		Some("H_CHAR"), Some("I_CHAR"), Some("M_CHAR"), Some("N_CHAR"), Some("Q_CHAR"), 
		Some("S_CHAR"), Some("U_CHAR"), Some("W_CHAR"), Some("X_CHAR"), Some("COMMENTTAG"), 
		Some("COMMACHAR"), Some("DOT"), Some("DOUBLEEQUALCHAR"), Some("NONNUMERICLITERAL"), 
		Some("NUMERICLITERAL"), Some("IDENTIFIER"), Some("FILENAME"), Some("NEWLINE"), 
		Some("COMMENTLINE"), Some("WS"), Some("TEXT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,Cobol85PreprocessorParserExt<'input>, I, Cobol85PreprocessorParserContextType , dyn Cobol85PreprocessorListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type Cobol85PreprocessorTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, Cobol85PreprocessorParserContextType , dyn Cobol85PreprocessorListener<'input> + 'a>;

/// Parser for Cobol85Preprocessor grammar
pub struct Cobol85PreprocessorParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				Cobol85PreprocessorParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> Cobol85PreprocessorParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> Cobol85PreprocessorParser<'input, I, DefaultErrorStrategy<'input,Cobol85PreprocessorParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for Cobol85PreprocessorParser
pub trait Cobol85PreprocessorParserContext<'input>:
	for<'x> Listenable<dyn Cobol85PreprocessorListener<'input> + 'x > + 
	for<'x> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=Cobol85PreprocessorParserContextType>
{}

antlr_rust::coerce_from!{ 'input : Cobol85PreprocessorParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn Cobol85PreprocessorParserContext<'input> + 'input
where
    T: Cobol85PreprocessorVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn Cobol85PreprocessorVisitor<'input> + 'x))
    }
}

impl<'input> Cobol85PreprocessorParserContext<'input> for TerminalNode<'input,Cobol85PreprocessorParserContextType> {}
impl<'input> Cobol85PreprocessorParserContext<'input> for ErrorNode<'input,Cobol85PreprocessorParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Cobol85PreprocessorParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Cobol85PreprocessorListener<'input> + 'input }

pub struct Cobol85PreprocessorParserContextType;
antlr_rust::tid!{Cobol85PreprocessorParserContextType}

impl<'input> ParserNodeType<'input> for Cobol85PreprocessorParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn Cobol85PreprocessorParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct Cobol85PreprocessorParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> Cobol85PreprocessorParserExt<'input>{
}
antlr_rust::tid! { Cobol85PreprocessorParserExt<'a> }

impl<'input> TokenAware<'input> for Cobol85PreprocessorParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for Cobol85PreprocessorParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for Cobol85PreprocessorParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Cobol85Preprocessor.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- startRule ----------------
pub type StartRuleContextAll<'input> = StartRuleContext<'input>;


pub type StartRuleContext<'input> = BaseParserRuleContext<'input,StartRuleContextExt<'input>>;

#[derive(Clone)]
pub struct StartRuleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for StartRuleContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for StartRuleContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_startRule(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_startRule(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for StartRuleContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_startRule(self);
	}
}

impl<'input> CustomRuleContext<'input> for StartRuleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startRule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startRule }
}
antlr_rust::tid!{StartRuleContextExt<'a>}

impl<'input> StartRuleContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartRuleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartRuleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartRuleContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<StartRuleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn compilerOptions_all(&self) ->  Vec<Rc<CompilerOptionsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compilerOptions(&self, i: usize) -> Option<Rc<CompilerOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn copyStatement_all(&self) ->  Vec<Rc<CopyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn copyStatement(&self, i: usize) -> Option<Rc<CopyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn execCicsStatement_all(&self) ->  Vec<Rc<ExecCicsStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn execCicsStatement(&self, i: usize) -> Option<Rc<ExecCicsStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn execSqlStatement_all(&self) ->  Vec<Rc<ExecSqlStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn execSqlStatement(&self, i: usize) -> Option<Rc<ExecSqlStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn execSqlImsStatement_all(&self) ->  Vec<Rc<ExecSqlImsStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn execSqlImsStatement(&self, i: usize) -> Option<Rc<ExecSqlImsStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn replaceOffStatement_all(&self) ->  Vec<Rc<ReplaceOffStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn replaceOffStatement(&self, i: usize) -> Option<Rc<ReplaceOffStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn replaceArea_all(&self) ->  Vec<Rc<ReplaceAreaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn replaceArea(&self, i: usize) -> Option<Rc<ReplaceAreaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn ejectStatement_all(&self) ->  Vec<Rc<EjectStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn ejectStatement(&self, i: usize) -> Option<Rc<EjectStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn skipStatement_all(&self) ->  Vec<Rc<SkipStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn skipStatement(&self, i: usize) -> Option<Rc<SkipStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn titleStatement_all(&self) ->  Vec<Rc<TitleStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn titleStatement(&self, i: usize) -> Option<Rc<TitleStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn charDataLine_all(&self) ->  Vec<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn charDataLine(&self, i: usize) -> Option<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> StartRuleContextAttrs<'input> for StartRuleContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn startRule(&mut self,)
	-> Result<Rc<StartRuleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartRuleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_startRule);
        let mut _localctx: Rc<StartRuleContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(74);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADATA) | (1usize << ADV) | (1usize << ALIAS) | (1usize << ANSI) | (1usize << ANY) | (1usize << APOST) | (1usize << AR) | (1usize << ARITH) | (1usize << AUTO) | (1usize << AWO) | (1usize << BIN) | (1usize << BLOCK0) | (1usize << BUF) | (1usize << BUFSIZE) | (1usize << BY) | (1usize << CBL) | (1usize << CBLCARD) | (1usize << CO) | (1usize << COBOL2) | (1usize << COBOL3) | (1usize << CODEPAGE) | (1usize << COMPAT) | (1usize << COMPILE) | (1usize << COPY) | (1usize << CP) | (1usize << CPP) | (1usize << CPSM) | (1usize << CS) | (1usize << CURR) | (1usize << CURRENCY))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (DATA - 32)) | (1usize << (DATEPROC - 32)) | (1usize << (DBCS - 32)) | (1usize << (DD - 32)) | (1usize << (DEBUG - 32)) | (1usize << (DECK - 32)) | (1usize << (DIAGTRUNC - 32)) | (1usize << (DLI - 32)) | (1usize << (DLL - 32)) | (1usize << (DP - 32)) | (1usize << (DTR - 32)) | (1usize << (DU - 32)) | (1usize << (DUMP - 32)) | (1usize << (DYN - 32)) | (1usize << (DYNAM - 32)) | (1usize << (EDF - 32)) | (1usize << (EJECT - 32)) | (1usize << (EJPD - 32)) | (1usize << (EN - 32)) | (1usize << (ENGLISH - 32)) | (1usize << (EPILOG - 32)) | (1usize << (EXCI - 32)) | (1usize << (EXEC - 32)) | (1usize << (EXIT - 32)) | (1usize << (EXP - 32)) | (1usize << (EXPORTALL - 32)) | (1usize << (EXTEND - 32)) | (1usize << (FASTSRT - 32)) | (1usize << (FLAG - 32)) | (1usize << (FLAGSTD - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (FSRT - 64)) | (1usize << (FULL - 64)) | (1usize << (GDS - 64)) | (1usize << (GRAPHIC - 64)) | (1usize << (HOOK - 64)) | (1usize << (IN - 64)) | (1usize << (INTDATE - 64)) | (1usize << (JA - 64)) | (1usize << (JP - 64)) | (1usize << (KA - 64)) | (1usize << (LANG - 64)) | (1usize << (LANGUAGE - 64)) | (1usize << (LC - 64)) | (1usize << (LENGTH - 64)) | (1usize << (LIB - 64)) | (1usize << (LILIAN - 64)) | (1usize << (LIN - 64)) | (1usize << (LINECOUNT - 64)) | (1usize << (LINKAGE - 64)) | (1usize << (LIST - 64)) | (1usize << (LM - 64)) | (1usize << (LONGMIXED - 64)) | (1usize << (LONGUPPER - 64)) | (1usize << (LPARENCHAR - 64)) | (1usize << (LU - 64)) | (1usize << (MAP - 64)) | (1usize << (MARGINS - 64)) | (1usize << (MAX - 64)) | (1usize << (MD - 64)) | (1usize << (MDECK - 64)) | (1usize << (MIG - 64)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MIXED - 96)) | (1usize << (NAME - 96)) | (1usize << (NAT - 96)) | (1usize << (NATIONAL - 96)) | (1usize << (NATLANG - 96)) | (1usize << (NN - 96)) | (1usize << (NO - 96)) | (1usize << (NOADATA - 96)) | (1usize << (NOADV - 96)) | (1usize << (NOALIAS - 96)) | (1usize << (NOAWO - 96)) | (1usize << (NOBLOCK0 - 96)) | (1usize << (NOC - 96)) | (1usize << (NOCBLCARD - 96)) | (1usize << (NOCICS - 96)) | (1usize << (NOCMPR2 - 96)) | (1usize << (NOCOMPILE - 96)) | (1usize << (NOCPSM - 96)) | (1usize << (NOCURR - 96)) | (1usize << (NOCURRENCY - 96)) | (1usize << (NOD - 96)) | (1usize << (NODATEPROC - 96)) | (1usize << (NODBCS - 96)) | (1usize << (NODE - 96)) | (1usize << (NODEBUG - 96)) | (1usize << (NODECK - 96)) | (1usize << (NODIAGTRUNC - 96)) | (1usize << (NODLL - 96)) | (1usize << (NODU - 96)) | (1usize << (NODUMP - 96)) | (1usize << (NODP - 96)) | (1usize << (NODTR - 96)))) != 0) || ((((_la - 128)) & !0x3f) == 0 && ((1usize << (_la - 128)) & ((1usize << (NODYN - 128)) | (1usize << (NODYNAM - 128)) | (1usize << (NOEDF - 128)) | (1usize << (NOEJPD - 128)) | (1usize << (NOEPILOG - 128)) | (1usize << (NOEXIT - 128)) | (1usize << (NOEXP - 128)) | (1usize << (NOEXPORTALL - 128)) | (1usize << (NOF - 128)) | (1usize << (NOFASTSRT - 128)) | (1usize << (NOFEPI - 128)) | (1usize << (NOFLAG - 128)) | (1usize << (NOFLAGMIG - 128)) | (1usize << (NOFLAGSTD - 128)) | (1usize << (NOFSRT - 128)) | (1usize << (NOGRAPHIC - 128)) | (1usize << (NOHOOK - 128)) | (1usize << (NOLENGTH - 128)) | (1usize << (NOLIB - 128)) | (1usize << (NOLINKAGE - 128)) | (1usize << (NOLIST - 128)) | (1usize << (NOMAP - 128)) | (1usize << (NOMD - 128)) | (1usize << (NOMDECK - 128)) | (1usize << (NONAME - 128)) | (1usize << (NONUM - 128)) | (1usize << (NONUMBER - 128)) | (1usize << (NOOBJ - 128)) | (1usize << (NOOBJECT - 128)) | (1usize << (NOOFF - 128)) | (1usize << (NOOFFSET - 128)) | (1usize << (NOOPSEQUENCE - 128)))) != 0) || ((((_la - 160)) & !0x3f) == 0 && ((1usize << (_la - 160)) & ((1usize << (NOOPT - 160)) | (1usize << (NOOPTIMIZE - 160)) | (1usize << (NOOPTIONS - 160)) | (1usize << (NOP - 160)) | (1usize << (NOPFD - 160)) | (1usize << (NOPROLOG - 160)) | (1usize << (NORENT - 160)) | (1usize << (NOS - 160)) | (1usize << (NOSEP - 160)) | (1usize << (NOSEPARATE - 160)) | (1usize << (NOSEQ - 160)) | (1usize << (NOSOURCE - 160)) | (1usize << (NOSPIE - 160)) | (1usize << (NOSQL - 160)) | (1usize << (NOSQLC - 160)) | (1usize << (NOSQLCCSID - 160)) | (1usize << (NOSSR - 160)) | (1usize << (NOSSRANGE - 160)) | (1usize << (NOSTDTRUNC - 160)) | (1usize << (NOSEQUENCE - 160)) | (1usize << (NOTERM - 160)) | (1usize << (NOTERMINAL - 160)) | (1usize << (NOTEST - 160)) | (1usize << (NOTHREAD - 160)) | (1usize << (NOTRIG - 160)) | (1usize << (NOVBREF - 160)) | (1usize << (NOWORD - 160)) | (1usize << (NOX - 160)) | (1usize << (NOXREF - 160)) | (1usize << (NOZWB - 160)) | (1usize << (NS - 160)))) != 0) || ((((_la - 192)) & !0x3f) == 0 && ((1usize << (_la - 192)) & ((1usize << (NSEQ - 192)) | (1usize << (NSYMBOL - 192)) | (1usize << (NUM - 192)) | (1usize << (NUMBER - 192)) | (1usize << (NUMPROC - 192)) | (1usize << (OBJ - 192)) | (1usize << (OBJECT - 192)) | (1usize << (OF - 192)) | (1usize << (OFF - 192)) | (1usize << (OFFSET - 192)) | (1usize << (ON - 192)) | (1usize << (OP - 192)) | (1usize << (OPMARGINS - 192)) | (1usize << (OPSEQUENCE - 192)) | (1usize << (OPT - 192)) | (1usize << (OPTFILE - 192)) | (1usize << (OPTIMIZE - 192)) | (1usize << (OPTIONS - 192)) | (1usize << (OUT - 192)) | (1usize << (OUTDD - 192)) | (1usize << (PFD - 192)) | (1usize << (PPTDBG - 192)) | (1usize << (PGMN - 192)) | (1usize << (PGMNAME - 192)) | (1usize << (PROCESS - 192)) | (1usize << (PROLOG - 192)) | (1usize << (QUOTE - 192)) | (1usize << (RENT - 192)) | (1usize << (REPLACE - 192)) | (1usize << (REPLACING - 192)) | (1usize << (RMODE - 192)) | (1usize << (RPARENCHAR - 192)))) != 0) || ((((_la - 224)) & !0x3f) == 0 && ((1usize << (_la - 224)) & ((1usize << (SEP - 224)) | (1usize << (SEPARATE - 224)) | (1usize << (SEQ - 224)) | (1usize << (SEQUENCE - 224)) | (1usize << (SHORT - 224)) | (1usize << (SIZE - 224)) | (1usize << (SOURCE - 224)) | (1usize << (SP - 224)) | (1usize << (SPACE - 224)) | (1usize << (SPIE - 224)) | (1usize << (SQL - 224)) | (1usize << (SQLC - 224)) | (1usize << (SQLCCSID - 224)) | (1usize << (SKIP1 - 224)) | (1usize << (SKIP2 - 224)) | (1usize << (SKIP3 - 224)) | (1usize << (SS - 224)) | (1usize << (SSR - 224)) | (1usize << (SSRANGE - 224)) | (1usize << (STD - 224)) | (1usize << (SYSEIB - 224)) | (1usize << (SZ - 224)) | (1usize << (TERM - 224)) | (1usize << (TERMINAL - 224)) | (1usize << (TEST - 224)) | (1usize << (THREAD - 224)) | (1usize << (TITLE - 224)) | (1usize << (TRIG - 224)) | (1usize << (TRUNC - 224)) | (1usize << (UE - 224)))) != 0) || ((((_la - 256)) & !0x3f) == 0 && ((1usize << (_la - 256)) & ((1usize << (UPPER - 256)) | (1usize << (VBREF - 256)) | (1usize << (WD - 256)) | (1usize << (XMLPARSE - 256)) | (1usize << (XMLSS - 256)) | (1usize << (XOPTS - 256)) | (1usize << (XREF - 256)) | (1usize << (YEARWINDOW - 256)) | (1usize << (YW - 256)) | (1usize << (ZWB - 256)) | (1usize << (C_CHAR - 256)) | (1usize << (D_CHAR - 256)) | (1usize << (E_CHAR - 256)) | (1usize << (F_CHAR - 256)) | (1usize << (H_CHAR - 256)) | (1usize << (I_CHAR - 256)) | (1usize << (M_CHAR - 256)) | (1usize << (N_CHAR - 256)) | (1usize << (Q_CHAR - 256)) | (1usize << (S_CHAR - 256)) | (1usize << (U_CHAR - 256)) | (1usize << (W_CHAR - 256)) | (1usize << (X_CHAR - 256)) | (1usize << (COMMACHAR - 256)) | (1usize << (DOT - 256)) | (1usize << (NONNUMERICLITERAL - 256)) | (1usize << (NUMERICLITERAL - 256)) | (1usize << (IDENTIFIER - 256)))) != 0) || ((((_la - 288)) & !0x3f) == 0 && ((1usize << (_la - 288)) & ((1usize << (FILENAME - 288)) | (1usize << (NEWLINE - 288)) | (1usize << (TEXT - 288)))) != 0) {
				{
				recog.base.set_state(72);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
					1 =>{
						{
						/*InvokeRule compilerOptions*/
						recog.base.set_state(60);
						recog.compilerOptions()?;

						}
					}
				,
					2 =>{
						{
						/*InvokeRule copyStatement*/
						recog.base.set_state(61);
						recog.copyStatement()?;

						}
					}
				,
					3 =>{
						{
						/*InvokeRule execCicsStatement*/
						recog.base.set_state(62);
						recog.execCicsStatement()?;

						}
					}
				,
					4 =>{
						{
						/*InvokeRule execSqlStatement*/
						recog.base.set_state(63);
						recog.execSqlStatement()?;

						}
					}
				,
					5 =>{
						{
						/*InvokeRule execSqlImsStatement*/
						recog.base.set_state(64);
						recog.execSqlImsStatement()?;

						}
					}
				,
					6 =>{
						{
						/*InvokeRule replaceOffStatement*/
						recog.base.set_state(65);
						recog.replaceOffStatement()?;

						}
					}
				,
					7 =>{
						{
						/*InvokeRule replaceArea*/
						recog.base.set_state(66);
						recog.replaceArea()?;

						}
					}
				,
					8 =>{
						{
						/*InvokeRule ejectStatement*/
						recog.base.set_state(67);
						recog.ejectStatement()?;

						}
					}
				,
					9 =>{
						{
						/*InvokeRule skipStatement*/
						recog.base.set_state(68);
						recog.skipStatement()?;

						}
					}
				,
					10 =>{
						{
						/*InvokeRule titleStatement*/
						recog.base.set_state(69);
						recog.titleStatement()?;

						}
					}
				,
					11 =>{
						{
						/*InvokeRule charDataLine*/
						recog.base.set_state(70);
						recog.charDataLine()?;

						}
					}
				,
					12 =>{
						{
						recog.base.set_state(71);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(76);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(77);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compilerOptions ----------------
pub type CompilerOptionsContextAll<'input> = CompilerOptionsContext<'input>;


pub type CompilerOptionsContext<'input> = BaseParserRuleContext<'input,CompilerOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct CompilerOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CompilerOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CompilerOptionsContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilerOptions(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_compilerOptions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CompilerOptionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_compilerOptions(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilerOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilerOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilerOptions }
}
antlr_rust::tid!{CompilerOptionsContextExt<'a>}

impl<'input> CompilerOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilerOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilerOptionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilerOptionsContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CompilerOptionsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PROCESS
/// Returns `None` if there is no child corresponding to token PROCESS
fn PROCESS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PROCESS, 0)
}
/// Retrieves first TerminalNode corresponding to token CBL
/// Returns `None` if there is no child corresponding to token CBL
fn CBL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CBL, 0)
}
fn compilerOption_all(&self) ->  Vec<Rc<CompilerOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compilerOption(&self, i: usize) -> Option<Rc<CompilerOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn compilerXOpts_all(&self) ->  Vec<Rc<CompilerXOptsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compilerXOpts(&self, i: usize) -> Option<Rc<CompilerXOptsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMACHAR in current rule
fn COMMACHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMACHAR, starting from 0.
/// Returns `None` if number of children corresponding to token COMMACHAR is less or equal than `i`.
fn COMMACHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMMACHAR, i)
}

}

impl<'input> CompilerOptionsContextAttrs<'input> for CompilerOptionsContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilerOptions(&mut self,)
	-> Result<Rc<CompilerOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilerOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_compilerOptions);
        let mut _localctx: Rc<CompilerOptionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(79);
			_la = recog.base.input.la(1);
			if { !(_la==CBL || _la==PROCESS) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(85); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					recog.base.set_state(85);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 ADATA | ADV | APOST | AR | ARITH | AWO | BLOCK0 | BUF | BUFSIZE | CBLCARD |
					 CICS | COBOL2 | COBOL3 | CODEPAGE | COMPILE | CP | CPP | CPSM | CURR |
					 CURRENCY | DATA | DATEPROC | DBCS | DEBUG | DECK | DIAGTRUNC | DLL |
					 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EPILOG | EXIT | EXP | EXPORTALL |
					 FASTSRT | FEPI | FLAG | FLAGSTD | FSRT | GDS | GRAPHIC | INTDATE |
					 LANG | LANGUAGE | LC | LEASM | LENGTH | LIB | LIN | LINECOUNT | LINKAGE |
					 LIST | MAP | MARGINS | MD | MDECK | NAME | NATLANG | NOADATA | NOADV |
					 NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
					 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
					 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
					 NODYNAM | NOEDF | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL | NOF | NOFASTSRT |
					 NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT | NOGRAPHIC | NOLENGTH |
					 NOLIB | NOLINKAGE | NOLIST | NOMAP | NOMD | NOMDECK | NONAME | NONUM |
					 NONUMBER | NOOBJ | NOOBJECT | NOOFF | NOOFFSET | NOOPSEQUENCE | NOOPT |
					 NOOPTIMIZE | NOOPTIONS | NOP | NOPROLOG | NORENT | NOS | NOSEQ | NOSOURCE |
					 NOSPIE | NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC |
					 NOSEQUENCE | NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOVBREF | NOWD |
					 NOWORD | NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER |
					 NUMPROC | OBJ | OBJECT | OFF | OFFSET | OP | OPMARGINS | OPSEQUENCE |
					 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PGMN | PGMNAME |
					 PROLOG | QUOTE | RENT | RMODE | SEQ | SEQUENCE | SIZE | SOURCE | SP |
					 SPACE | SPIE | SQL | SQLC | SQLCCSID | SSR | SSRANGE | SYSEIB | SZ |
					 TERM | TERMINAL | TEST | THREAD | TRUNC | VBREF | WD | WORD | XMLPARSE |
					 XP | XREF | YEARWINDOW | YW | ZWB | C_CHAR | D_CHAR | F_CHAR | Q_CHAR |
					 S_CHAR | X_CHAR | COMMACHAR 
						=> {
							{
							recog.base.set_state(81);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==COMMACHAR {
								{
								recog.base.set_state(80);
								recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

								}
							}

							/*InvokeRule compilerOption*/
							recog.base.set_state(83);
							recog.compilerOption()?;

							}
						}

					 XOPTS 
						=> {
							{
							/*InvokeRule compilerXOpts*/
							recog.base.set_state(84);
							recog.compilerXOpts()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(87); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compilerXOpts ----------------
pub type CompilerXOptsContextAll<'input> = CompilerXOptsContext<'input>;


pub type CompilerXOptsContext<'input> = BaseParserRuleContext<'input,CompilerXOptsContextExt<'input>>;

#[derive(Clone)]
pub struct CompilerXOptsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CompilerXOptsContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CompilerXOptsContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilerXOpts(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_compilerXOpts(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CompilerXOptsContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_compilerXOpts(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilerXOptsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilerXOpts }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilerXOpts }
}
antlr_rust::tid!{CompilerXOptsContextExt<'a>}

impl<'input> CompilerXOptsContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilerXOptsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilerXOptsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilerXOptsContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CompilerXOptsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token XOPTS
/// Returns `None` if there is no child corresponding to token XOPTS
fn XOPTS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XOPTS, 0)
}
/// Retrieves first TerminalNode corresponding to token LPARENCHAR
/// Returns `None` if there is no child corresponding to token LPARENCHAR
fn LPARENCHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LPARENCHAR, 0)
}
fn compilerOption_all(&self) ->  Vec<Rc<CompilerOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compilerOption(&self, i: usize) -> Option<Rc<CompilerOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPARENCHAR
/// Returns `None` if there is no child corresponding to token RPARENCHAR
fn RPARENCHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RPARENCHAR, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMACHAR in current rule
fn COMMACHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMACHAR, starting from 0.
/// Returns `None` if number of children corresponding to token COMMACHAR is less or equal than `i`.
fn COMMACHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMMACHAR, i)
}

}

impl<'input> CompilerXOptsContextAttrs<'input> for CompilerXOptsContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilerXOpts(&mut self,)
	-> Result<Rc<CompilerXOptsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilerXOptsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_compilerXOpts);
        let mut _localctx: Rc<CompilerXOptsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(89);
			recog.base.match_token(XOPTS,&mut recog.err_handler)?;

			recog.base.set_state(90);
			recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

			/*InvokeRule compilerOption*/
			recog.base.set_state(91);
			recog.compilerOption()?;

			recog.base.set_state(98);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADATA) | (1usize << ADV) | (1usize << APOST) | (1usize << AR) | (1usize << ARITH) | (1usize << AWO) | (1usize << BLOCK0) | (1usize << BUF) | (1usize << BUFSIZE) | (1usize << CBLCARD) | (1usize << CICS) | (1usize << COBOL2) | (1usize << COBOL3) | (1usize << CODEPAGE) | (1usize << COMPILE) | (1usize << CP) | (1usize << CPP) | (1usize << CPSM) | (1usize << CURR) | (1usize << CURRENCY))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (DATA - 32)) | (1usize << (DATEPROC - 32)) | (1usize << (DBCS - 32)) | (1usize << (DEBUG - 32)) | (1usize << (DECK - 32)) | (1usize << (DIAGTRUNC - 32)) | (1usize << (DLL - 32)) | (1usize << (DP - 32)) | (1usize << (DTR - 32)) | (1usize << (DU - 32)) | (1usize << (DUMP - 32)) | (1usize << (DYN - 32)) | (1usize << (DYNAM - 32)) | (1usize << (EDF - 32)) | (1usize << (EPILOG - 32)) | (1usize << (EXIT - 32)) | (1usize << (EXP - 32)) | (1usize << (EXPORTALL - 32)) | (1usize << (FASTSRT - 32)) | (1usize << (FEPI - 32)) | (1usize << (FLAG - 32)) | (1usize << (FLAGSTD - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (FSRT - 64)) | (1usize << (GDS - 64)) | (1usize << (GRAPHIC - 64)) | (1usize << (INTDATE - 64)) | (1usize << (LANG - 64)) | (1usize << (LANGUAGE - 64)) | (1usize << (LC - 64)) | (1usize << (LEASM - 64)) | (1usize << (LENGTH - 64)) | (1usize << (LIB - 64)) | (1usize << (LIN - 64)) | (1usize << (LINECOUNT - 64)) | (1usize << (LINKAGE - 64)) | (1usize << (LIST - 64)) | (1usize << (MAP - 64)) | (1usize << (MARGINS - 64)) | (1usize << (MD - 64)) | (1usize << (MDECK - 64)))) != 0) || ((((_la - 97)) & !0x3f) == 0 && ((1usize << (_la - 97)) & ((1usize << (NAME - 97)) | (1usize << (NATLANG - 97)) | (1usize << (NOADATA - 97)) | (1usize << (NOADV - 97)) | (1usize << (NOAWO - 97)) | (1usize << (NOBLOCK0 - 97)) | (1usize << (NOC - 97)) | (1usize << (NOCBLCARD - 97)) | (1usize << (NOCICS - 97)) | (1usize << (NOCMPR2 - 97)) | (1usize << (NOCOMPILE - 97)) | (1usize << (NOCPSM - 97)) | (1usize << (NOCURR - 97)) | (1usize << (NOCURRENCY - 97)) | (1usize << (NOD - 97)) | (1usize << (NODATEPROC - 97)) | (1usize << (NODBCS - 97)) | (1usize << (NODE - 97)) | (1usize << (NODEBUG - 97)) | (1usize << (NODECK - 97)) | (1usize << (NODIAGTRUNC - 97)) | (1usize << (NODLL - 97)) | (1usize << (NODU - 97)) | (1usize << (NODUMP - 97)) | (1usize << (NODP - 97)) | (1usize << (NODTR - 97)) | (1usize << (NODYN - 97)))) != 0) || ((((_la - 129)) & !0x3f) == 0 && ((1usize << (_la - 129)) & ((1usize << (NODYNAM - 129)) | (1usize << (NOEDF - 129)) | (1usize << (NOEPILOG - 129)) | (1usize << (NOEXIT - 129)) | (1usize << (NOEXP - 129)) | (1usize << (NOEXPORTALL - 129)) | (1usize << (NOF - 129)) | (1usize << (NOFASTSRT - 129)) | (1usize << (NOFEPI - 129)) | (1usize << (NOFLAG - 129)) | (1usize << (NOFLAGMIG - 129)) | (1usize << (NOFLAGSTD - 129)) | (1usize << (NOFSRT - 129)) | (1usize << (NOGRAPHIC - 129)) | (1usize << (NOLENGTH - 129)) | (1usize << (NOLIB - 129)) | (1usize << (NOLINKAGE - 129)) | (1usize << (NOLIST - 129)) | (1usize << (NOMAP - 129)) | (1usize << (NOMD - 129)) | (1usize << (NOMDECK - 129)) | (1usize << (NONAME - 129)) | (1usize << (NONUM - 129)) | (1usize << (NONUMBER - 129)) | (1usize << (NOOBJ - 129)) | (1usize << (NOOBJECT - 129)) | (1usize << (NOOFF - 129)) | (1usize << (NOOFFSET - 129)) | (1usize << (NOOPSEQUENCE - 129)) | (1usize << (NOOPT - 129)))) != 0) || ((((_la - 161)) & !0x3f) == 0 && ((1usize << (_la - 161)) & ((1usize << (NOOPTIMIZE - 161)) | (1usize << (NOOPTIONS - 161)) | (1usize << (NOP - 161)) | (1usize << (NOPROLOG - 161)) | (1usize << (NORENT - 161)) | (1usize << (NOS - 161)) | (1usize << (NOSEQ - 161)) | (1usize << (NOSOURCE - 161)) | (1usize << (NOSPIE - 161)) | (1usize << (NOSQL - 161)) | (1usize << (NOSQLC - 161)) | (1usize << (NOSQLCCSID - 161)) | (1usize << (NOSSR - 161)) | (1usize << (NOSSRANGE - 161)) | (1usize << (NOSTDTRUNC - 161)) | (1usize << (NOSEQUENCE - 161)) | (1usize << (NOTERM - 161)) | (1usize << (NOTERMINAL - 161)) | (1usize << (NOTEST - 161)) | (1usize << (NOTHREAD - 161)) | (1usize << (NOVBREF - 161)) | (1usize << (NOWD - 161)) | (1usize << (NOWORD - 161)) | (1usize << (NOX - 161)) | (1usize << (NOXREF - 161)) | (1usize << (NOZWB - 161)) | (1usize << (NS - 161)) | (1usize << (NSEQ - 161)))) != 0) || ((((_la - 193)) & !0x3f) == 0 && ((1usize << (_la - 193)) & ((1usize << (NSYMBOL - 193)) | (1usize << (NUM - 193)) | (1usize << (NUMBER - 193)) | (1usize << (NUMPROC - 193)) | (1usize << (OBJ - 193)) | (1usize << (OBJECT - 193)) | (1usize << (OFF - 193)) | (1usize << (OFFSET - 193)) | (1usize << (OP - 193)) | (1usize << (OPMARGINS - 193)) | (1usize << (OPSEQUENCE - 193)) | (1usize << (OPT - 193)) | (1usize << (OPTFILE - 193)) | (1usize << (OPTIMIZE - 193)) | (1usize << (OPTIONS - 193)) | (1usize << (OUT - 193)) | (1usize << (OUTDD - 193)) | (1usize << (PGMN - 193)) | (1usize << (PGMNAME - 193)) | (1usize << (PROLOG - 193)) | (1usize << (QUOTE - 193)) | (1usize << (RENT - 193)) | (1usize << (RMODE - 193)))) != 0) || ((((_la - 226)) & !0x3f) == 0 && ((1usize << (_la - 226)) & ((1usize << (SEQ - 226)) | (1usize << (SEQUENCE - 226)) | (1usize << (SIZE - 226)) | (1usize << (SOURCE - 226)) | (1usize << (SP - 226)) | (1usize << (SPACE - 226)) | (1usize << (SPIE - 226)) | (1usize << (SQL - 226)) | (1usize << (SQLC - 226)) | (1usize << (SQLCCSID - 226)) | (1usize << (SSR - 226)) | (1usize << (SSRANGE - 226)) | (1usize << (SYSEIB - 226)) | (1usize << (SZ - 226)) | (1usize << (TERM - 226)) | (1usize << (TERMINAL - 226)) | (1usize << (TEST - 226)) | (1usize << (THREAD - 226)) | (1usize << (TRUNC - 226)) | (1usize << (VBREF - 226)))) != 0) || ((((_la - 258)) & !0x3f) == 0 && ((1usize << (_la - 258)) & ((1usize << (WD - 258)) | (1usize << (WORD - 258)) | (1usize << (XMLPARSE - 258)) | (1usize << (XP - 258)) | (1usize << (XREF - 258)) | (1usize << (YEARWINDOW - 258)) | (1usize << (YW - 258)) | (1usize << (ZWB - 258)) | (1usize << (C_CHAR - 258)) | (1usize << (D_CHAR - 258)) | (1usize << (F_CHAR - 258)) | (1usize << (Q_CHAR - 258)) | (1usize << (S_CHAR - 258)) | (1usize << (X_CHAR - 258)) | (1usize << (COMMACHAR - 258)))) != 0) {
				{
				{
				recog.base.set_state(93);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==COMMACHAR {
					{
					recog.base.set_state(92);
					recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule compilerOption*/
				recog.base.set_state(95);
				recog.compilerOption()?;

				}
				}
				recog.base.set_state(100);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(101);
			recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compilerOption ----------------
pub type CompilerOptionContextAll<'input> = CompilerOptionContext<'input>;


pub type CompilerOptionContext<'input> = BaseParserRuleContext<'input,CompilerOptionContextExt<'input>>;

#[derive(Clone)]
pub struct CompilerOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CompilerOptionContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CompilerOptionContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilerOption(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_compilerOption(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CompilerOptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_compilerOption(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompilerOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilerOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilerOption }
}
antlr_rust::tid!{CompilerOptionContextExt<'a>}

impl<'input> CompilerOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilerOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilerOptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilerOptionContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CompilerOptionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADATA
/// Returns `None` if there is no child corresponding to token ADATA
fn ADATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ADATA, 0)
}
/// Retrieves first TerminalNode corresponding to token ADV
/// Returns `None` if there is no child corresponding to token ADV
fn ADV(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ADV, 0)
}
/// Retrieves first TerminalNode corresponding to token APOST
/// Returns `None` if there is no child corresponding to token APOST
fn APOST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(APOST, 0)
}
/// Retrieves first TerminalNode corresponding to token LPARENCHAR
/// Returns `None` if there is no child corresponding to token LPARENCHAR
fn LPARENCHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LPARENCHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RPARENCHAR
/// Returns `None` if there is no child corresponding to token RPARENCHAR
fn RPARENCHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RPARENCHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token ARITH
/// Returns `None` if there is no child corresponding to token ARITH
fn ARITH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ARITH, 0)
}
/// Retrieves first TerminalNode corresponding to token AR
/// Returns `None` if there is no child corresponding to token AR
fn AR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AR, 0)
}
/// Retrieves first TerminalNode corresponding to token EXTEND
/// Returns `None` if there is no child corresponding to token EXTEND
fn EXTEND(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXTEND, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token E_CHAR in current rule
fn E_CHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token E_CHAR, starting from 0.
/// Returns `None` if number of children corresponding to token E_CHAR is less or equal than `i`.
fn E_CHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(E_CHAR, i)
}
/// Retrieves first TerminalNode corresponding to token COMPAT
/// Returns `None` if there is no child corresponding to token COMPAT
fn COMPAT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMPAT, 0)
}
/// Retrieves first TerminalNode corresponding to token C_CHAR
/// Returns `None` if there is no child corresponding to token C_CHAR
fn C_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(C_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token AWO
/// Returns `None` if there is no child corresponding to token AWO
fn AWO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AWO, 0)
}
/// Retrieves first TerminalNode corresponding to token BLOCK0
/// Returns `None` if there is no child corresponding to token BLOCK0
fn BLOCK0(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BLOCK0, 0)
}
fn literal_all(&self) ->  Vec<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literal(&self, i: usize) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token BUFSIZE
/// Returns `None` if there is no child corresponding to token BUFSIZE
fn BUFSIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BUFSIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token BUF
/// Returns `None` if there is no child corresponding to token BUF
fn BUF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BUF, 0)
}
/// Retrieves first TerminalNode corresponding to token CBLCARD
/// Returns `None` if there is no child corresponding to token CBLCARD
fn CBLCARD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CBLCARD, 0)
}
/// Retrieves first TerminalNode corresponding to token CICS
/// Returns `None` if there is no child corresponding to token CICS
fn CICS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CICS, 0)
}
/// Retrieves first TerminalNode corresponding to token COBOL2
/// Returns `None` if there is no child corresponding to token COBOL2
fn COBOL2(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COBOL2, 0)
}
/// Retrieves first TerminalNode corresponding to token COBOL3
/// Returns `None` if there is no child corresponding to token COBOL3
fn COBOL3(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COBOL3, 0)
}
/// Retrieves first TerminalNode corresponding to token CODEPAGE
/// Returns `None` if there is no child corresponding to token CODEPAGE
fn CODEPAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CODEPAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token CP
/// Returns `None` if there is no child corresponding to token CP
fn CP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CP, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPILE
/// Returns `None` if there is no child corresponding to token COMPILE
fn COMPILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMPILE, 0)
}
/// Retrieves first TerminalNode corresponding to token CPP
/// Returns `None` if there is no child corresponding to token CPP
fn CPP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CPP, 0)
}
/// Retrieves first TerminalNode corresponding to token CPSM
/// Returns `None` if there is no child corresponding to token CPSM
fn CPSM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CPSM, 0)
}
/// Retrieves first TerminalNode corresponding to token CURRENCY
/// Returns `None` if there is no child corresponding to token CURRENCY
fn CURRENCY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CURRENCY, 0)
}
/// Retrieves first TerminalNode corresponding to token CURR
/// Returns `None` if there is no child corresponding to token CURR
fn CURR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CURR, 0)
}
/// Retrieves first TerminalNode corresponding to token DATA
/// Returns `None` if there is no child corresponding to token DATA
fn DATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DATA, 0)
}
/// Retrieves first TerminalNode corresponding to token DATEPROC
/// Returns `None` if there is no child corresponding to token DATEPROC
fn DATEPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DATEPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token DP
/// Returns `None` if there is no child corresponding to token DP
fn DP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DP, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMACHAR in current rule
fn COMMACHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMACHAR, starting from 0.
/// Returns `None` if number of children corresponding to token COMMACHAR is less or equal than `i`.
fn COMMACHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMMACHAR, i)
}
/// Retrieves first TerminalNode corresponding to token FLAG
/// Returns `None` if there is no child corresponding to token FLAG
fn FLAG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FLAG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAG
/// Returns `None` if there is no child corresponding to token NOFLAG
fn NOFLAG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAG, 0)
}
/// Retrieves first TerminalNode corresponding to token TRIG
/// Returns `None` if there is no child corresponding to token TRIG
fn TRIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TRIG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTRIG
/// Returns `None` if there is no child corresponding to token NOTRIG
fn NOTRIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTRIG, 0)
}
/// Retrieves first TerminalNode corresponding to token DBCS
/// Returns `None` if there is no child corresponding to token DBCS
fn DBCS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DBCS, 0)
}
/// Retrieves first TerminalNode corresponding to token DECK
/// Returns `None` if there is no child corresponding to token DECK
fn DECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DECK, 0)
}
/// Retrieves first TerminalNode corresponding to token D_CHAR
/// Returns `None` if there is no child corresponding to token D_CHAR
fn D_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(D_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token DEBUG
/// Returns `None` if there is no child corresponding to token DEBUG
fn DEBUG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DEBUG, 0)
}
/// Retrieves first TerminalNode corresponding to token DIAGTRUNC
/// Returns `None` if there is no child corresponding to token DIAGTRUNC
fn DIAGTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DIAGTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token DTR
/// Returns `None` if there is no child corresponding to token DTR
fn DTR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DTR, 0)
}
/// Retrieves first TerminalNode corresponding to token DLL
/// Returns `None` if there is no child corresponding to token DLL
fn DLL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DLL, 0)
}
/// Retrieves first TerminalNode corresponding to token DUMP
/// Returns `None` if there is no child corresponding to token DUMP
fn DUMP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DUMP, 0)
}
/// Retrieves first TerminalNode corresponding to token DU
/// Returns `None` if there is no child corresponding to token DU
fn DU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DU, 0)
}
/// Retrieves first TerminalNode corresponding to token DYNAM
/// Returns `None` if there is no child corresponding to token DYNAM
fn DYNAM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DYNAM, 0)
}
/// Retrieves first TerminalNode corresponding to token DYN
/// Returns `None` if there is no child corresponding to token DYN
fn DYN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DYN, 0)
}
/// Retrieves first TerminalNode corresponding to token EDF
/// Returns `None` if there is no child corresponding to token EDF
fn EDF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EDF, 0)
}
/// Retrieves first TerminalNode corresponding to token EPILOG
/// Returns `None` if there is no child corresponding to token EPILOG
fn EPILOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EPILOG, 0)
}
/// Retrieves first TerminalNode corresponding to token EXIT
/// Returns `None` if there is no child corresponding to token EXIT
fn EXIT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXIT, 0)
}
/// Retrieves first TerminalNode corresponding to token EXPORTALL
/// Returns `None` if there is no child corresponding to token EXPORTALL
fn EXPORTALL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXPORTALL, 0)
}
/// Retrieves first TerminalNode corresponding to token EXP
/// Returns `None` if there is no child corresponding to token EXP
fn EXP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXP, 0)
}
/// Retrieves first TerminalNode corresponding to token FASTSRT
/// Returns `None` if there is no child corresponding to token FASTSRT
fn FASTSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FASTSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token FSRT
/// Returns `None` if there is no child corresponding to token FSRT
fn FSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token FEPI
/// Returns `None` if there is no child corresponding to token FEPI
fn FEPI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FEPI, 0)
}
/// Retrieves first TerminalNode corresponding to token F_CHAR
/// Returns `None` if there is no child corresponding to token F_CHAR
fn F_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(F_CHAR, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token I_CHAR in current rule
fn I_CHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token I_CHAR, starting from 0.
/// Returns `None` if number of children corresponding to token I_CHAR is less or equal than `i`.
fn I_CHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(I_CHAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token S_CHAR in current rule
fn S_CHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token S_CHAR, starting from 0.
/// Returns `None` if number of children corresponding to token S_CHAR is less or equal than `i`.
fn S_CHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(S_CHAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token U_CHAR in current rule
fn U_CHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token U_CHAR, starting from 0.
/// Returns `None` if number of children corresponding to token U_CHAR is less or equal than `i`.
fn U_CHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(U_CHAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token W_CHAR in current rule
fn W_CHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token W_CHAR, starting from 0.
/// Returns `None` if number of children corresponding to token W_CHAR is less or equal than `i`.
fn W_CHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(W_CHAR, i)
}
/// Retrieves first TerminalNode corresponding to token FLAGSTD
/// Returns `None` if there is no child corresponding to token FLAGSTD
fn FLAGSTD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FLAGSTD, 0)
}
/// Retrieves first TerminalNode corresponding to token M_CHAR
/// Returns `None` if there is no child corresponding to token M_CHAR
fn M_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(M_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token H_CHAR
/// Returns `None` if there is no child corresponding to token H_CHAR
fn H_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(H_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token DD
/// Returns `None` if there is no child corresponding to token DD
fn DD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DD, 0)
}
/// Retrieves first TerminalNode corresponding to token N_CHAR
/// Returns `None` if there is no child corresponding to token N_CHAR
fn N_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(N_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token NN
/// Returns `None` if there is no child corresponding to token NN
fn NN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NN, 0)
}
/// Retrieves first TerminalNode corresponding to token SS
/// Returns `None` if there is no child corresponding to token SS
fn SS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SS, 0)
}
/// Retrieves first TerminalNode corresponding to token GDS
/// Returns `None` if there is no child corresponding to token GDS
fn GDS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(GDS, 0)
}
/// Retrieves first TerminalNode corresponding to token GRAPHIC
/// Returns `None` if there is no child corresponding to token GRAPHIC
fn GRAPHIC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(GRAPHIC, 0)
}
/// Retrieves first TerminalNode corresponding to token INTDATE
/// Returns `None` if there is no child corresponding to token INTDATE
fn INTDATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(INTDATE, 0)
}
/// Retrieves first TerminalNode corresponding to token ANSI
/// Returns `None` if there is no child corresponding to token ANSI
fn ANSI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ANSI, 0)
}
/// Retrieves first TerminalNode corresponding to token LILIAN
/// Returns `None` if there is no child corresponding to token LILIAN
fn LILIAN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LILIAN, 0)
}
/// Retrieves first TerminalNode corresponding to token LANGUAGE
/// Returns `None` if there is no child corresponding to token LANGUAGE
fn LANGUAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LANGUAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token LANG
/// Returns `None` if there is no child corresponding to token LANG
fn LANG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LANG, 0)
}
/// Retrieves first TerminalNode corresponding to token ENGLISH
/// Returns `None` if there is no child corresponding to token ENGLISH
fn ENGLISH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ENGLISH, 0)
}
/// Retrieves first TerminalNode corresponding to token CS
/// Returns `None` if there is no child corresponding to token CS
fn CS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CS, 0)
}
/// Retrieves first TerminalNode corresponding to token EN
/// Returns `None` if there is no child corresponding to token EN
fn EN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EN, 0)
}
/// Retrieves first TerminalNode corresponding to token JA
/// Returns `None` if there is no child corresponding to token JA
fn JA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(JA, 0)
}
/// Retrieves first TerminalNode corresponding to token JP
/// Returns `None` if there is no child corresponding to token JP
fn JP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(JP, 0)
}
/// Retrieves first TerminalNode corresponding to token KA
/// Returns `None` if there is no child corresponding to token KA
fn KA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(KA, 0)
}
/// Retrieves first TerminalNode corresponding to token UE
/// Returns `None` if there is no child corresponding to token UE
fn UE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(UE, 0)
}
/// Retrieves first TerminalNode corresponding to token LEASM
/// Returns `None` if there is no child corresponding to token LEASM
fn LEASM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LEASM, 0)
}
/// Retrieves first TerminalNode corresponding to token LENGTH
/// Returns `None` if there is no child corresponding to token LENGTH
fn LENGTH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token LIB
/// Returns `None` if there is no child corresponding to token LIB
fn LIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIB, 0)
}
/// Retrieves first TerminalNode corresponding to token LIN
/// Returns `None` if there is no child corresponding to token LIN
fn LIN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIN, 0)
}
/// Retrieves first TerminalNode corresponding to token LINECOUNT
/// Returns `None` if there is no child corresponding to token LINECOUNT
fn LINECOUNT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LINECOUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token LC
/// Returns `None` if there is no child corresponding to token LC
fn LC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LC, 0)
}
/// Retrieves first TerminalNode corresponding to token LINKAGE
/// Returns `None` if there is no child corresponding to token LINKAGE
fn LINKAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LINKAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token LIST
/// Returns `None` if there is no child corresponding to token LIST
fn LIST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIST, 0)
}
/// Retrieves first TerminalNode corresponding to token MAP
/// Returns `None` if there is no child corresponding to token MAP
fn MAP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MAP, 0)
}
/// Retrieves first TerminalNode corresponding to token MARGINS
/// Returns `None` if there is no child corresponding to token MARGINS
fn MARGINS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MARGINS, 0)
}
/// Retrieves first TerminalNode corresponding to token MDECK
/// Returns `None` if there is no child corresponding to token MDECK
fn MDECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MDECK, 0)
}
/// Retrieves first TerminalNode corresponding to token MD
/// Returns `None` if there is no child corresponding to token MD
fn MD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOC
/// Returns `None` if there is no child corresponding to token NOC
fn NOC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCOMPILE
/// Returns `None` if there is no child corresponding to token NOCOMPILE
fn NOCOMPILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCOMPILE, 0)
}
/// Retrieves first TerminalNode corresponding to token NAME
/// Returns `None` if there is no child corresponding to token NAME
fn NAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token ALIAS
/// Returns `None` if there is no child corresponding to token ALIAS
fn ALIAS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ALIAS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOALIAS
/// Returns `None` if there is no child corresponding to token NOALIAS
fn NOALIAS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOALIAS, 0)
}
/// Retrieves first TerminalNode corresponding to token NATLANG
/// Returns `None` if there is no child corresponding to token NATLANG
fn NATLANG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NATLANG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOADATA
/// Returns `None` if there is no child corresponding to token NOADATA
fn NOADATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOADATA, 0)
}
/// Retrieves first TerminalNode corresponding to token NOADV
/// Returns `None` if there is no child corresponding to token NOADV
fn NOADV(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOADV, 0)
}
/// Retrieves first TerminalNode corresponding to token NOAWO
/// Returns `None` if there is no child corresponding to token NOAWO
fn NOAWO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOAWO, 0)
}
/// Retrieves first TerminalNode corresponding to token NOBLOCK0
/// Returns `None` if there is no child corresponding to token NOBLOCK0
fn NOBLOCK0(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOBLOCK0, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCBLCARD
/// Returns `None` if there is no child corresponding to token NOCBLCARD
fn NOCBLCARD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCBLCARD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCICS
/// Returns `None` if there is no child corresponding to token NOCICS
fn NOCICS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCICS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCMPR2
/// Returns `None` if there is no child corresponding to token NOCMPR2
fn NOCMPR2(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCMPR2, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCPSM
/// Returns `None` if there is no child corresponding to token NOCPSM
fn NOCPSM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCPSM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCURRENCY
/// Returns `None` if there is no child corresponding to token NOCURRENCY
fn NOCURRENCY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCURRENCY, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCURR
/// Returns `None` if there is no child corresponding to token NOCURR
fn NOCURR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCURR, 0)
}
/// Retrieves first TerminalNode corresponding to token NODATEPROC
/// Returns `None` if there is no child corresponding to token NODATEPROC
fn NODATEPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODATEPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token NODP
/// Returns `None` if there is no child corresponding to token NODP
fn NODP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODP, 0)
}
/// Retrieves first TerminalNode corresponding to token NODBCS
/// Returns `None` if there is no child corresponding to token NODBCS
fn NODBCS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODBCS, 0)
}
/// Retrieves first TerminalNode corresponding to token NODEBUG
/// Returns `None` if there is no child corresponding to token NODEBUG
fn NODEBUG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODEBUG, 0)
}
/// Retrieves first TerminalNode corresponding to token NODECK
/// Returns `None` if there is no child corresponding to token NODECK
fn NODECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODECK, 0)
}
/// Retrieves first TerminalNode corresponding to token NOD
/// Returns `None` if there is no child corresponding to token NOD
fn NOD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOD, 0)
}
/// Retrieves first TerminalNode corresponding to token NODLL
/// Returns `None` if there is no child corresponding to token NODLL
fn NODLL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODLL, 0)
}
/// Retrieves first TerminalNode corresponding to token NODE
/// Returns `None` if there is no child corresponding to token NODE
fn NODE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODE, 0)
}
/// Retrieves first TerminalNode corresponding to token NODUMP
/// Returns `None` if there is no child corresponding to token NODUMP
fn NODUMP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODUMP, 0)
}
/// Retrieves first TerminalNode corresponding to token NODU
/// Returns `None` if there is no child corresponding to token NODU
fn NODU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODU, 0)
}
/// Retrieves first TerminalNode corresponding to token NODIAGTRUNC
/// Returns `None` if there is no child corresponding to token NODIAGTRUNC
fn NODIAGTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODIAGTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token NODTR
/// Returns `None` if there is no child corresponding to token NODTR
fn NODTR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODTR, 0)
}
/// Retrieves first TerminalNode corresponding to token NODYNAM
/// Returns `None` if there is no child corresponding to token NODYNAM
fn NODYNAM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODYNAM, 0)
}
/// Retrieves first TerminalNode corresponding to token NODYN
/// Returns `None` if there is no child corresponding to token NODYN
fn NODYN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODYN, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEDF
/// Returns `None` if there is no child corresponding to token NOEDF
fn NOEDF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEDF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEPILOG
/// Returns `None` if there is no child corresponding to token NOEPILOG
fn NOEPILOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEPILOG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXIT
/// Returns `None` if there is no child corresponding to token NOEXIT
fn NOEXIT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXIT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXPORTALL
/// Returns `None` if there is no child corresponding to token NOEXPORTALL
fn NOEXPORTALL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXPORTALL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXP
/// Returns `None` if there is no child corresponding to token NOEXP
fn NOEXP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFASTSRT
/// Returns `None` if there is no child corresponding to token NOFASTSRT
fn NOFASTSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFASTSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFSRT
/// Returns `None` if there is no child corresponding to token NOFSRT
fn NOFSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFEPI
/// Returns `None` if there is no child corresponding to token NOFEPI
fn NOFEPI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFEPI, 0)
}
/// Retrieves first TerminalNode corresponding to token NOF
/// Returns `None` if there is no child corresponding to token NOF
fn NOF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAGMIG
/// Returns `None` if there is no child corresponding to token NOFLAGMIG
fn NOFLAGMIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAGMIG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAGSTD
/// Returns `None` if there is no child corresponding to token NOFLAGSTD
fn NOFLAGSTD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAGSTD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOGRAPHIC
/// Returns `None` if there is no child corresponding to token NOGRAPHIC
fn NOGRAPHIC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOGRAPHIC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLENGTH
/// Returns `None` if there is no child corresponding to token NOLENGTH
fn NOLENGTH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLIB
/// Returns `None` if there is no child corresponding to token NOLIB
fn NOLIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLIB, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLINKAGE
/// Returns `None` if there is no child corresponding to token NOLINKAGE
fn NOLINKAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLINKAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLIST
/// Returns `None` if there is no child corresponding to token NOLIST
fn NOLIST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLIST, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMAP
/// Returns `None` if there is no child corresponding to token NOMAP
fn NOMAP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMAP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMDECK
/// Returns `None` if there is no child corresponding to token NOMDECK
fn NOMDECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMDECK, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMD
/// Returns `None` if there is no child corresponding to token NOMD
fn NOMD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMD, 0)
}
/// Retrieves first TerminalNode corresponding to token NONAME
/// Returns `None` if there is no child corresponding to token NONAME
fn NONAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONAME, 0)
}
/// Retrieves first TerminalNode corresponding to token NONUMBER
/// Returns `None` if there is no child corresponding to token NONUMBER
fn NONUMBER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NONUM
/// Returns `None` if there is no child corresponding to token NONUM
fn NONUM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONUM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOBJECT
/// Returns `None` if there is no child corresponding to token NOOBJECT
fn NOOBJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOBJ
/// Returns `None` if there is no child corresponding to token NOOBJ
fn NOOBJ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOBJ, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOFFSET
/// Returns `None` if there is no child corresponding to token NOOFFSET
fn NOOFFSET(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOFF
/// Returns `None` if there is no child corresponding to token NOOFF
fn NOOFF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOFF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPSEQUENCE
/// Returns `None` if there is no child corresponding to token NOOPSEQUENCE
fn NOOPSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPTIMIZE
/// Returns `None` if there is no child corresponding to token NOOPTIMIZE
fn NOOPTIMIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPTIMIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPT
/// Returns `None` if there is no child corresponding to token NOOPT
fn NOOPT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPTIONS
/// Returns `None` if there is no child corresponding to token NOOPTIONS
fn NOOPTIONS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPTIONS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOP
/// Returns `None` if there is no child corresponding to token NOP
fn NOP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOPROLOG
/// Returns `None` if there is no child corresponding to token NOPROLOG
fn NOPROLOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOPROLOG, 0)
}
/// Retrieves first TerminalNode corresponding to token NORENT
/// Returns `None` if there is no child corresponding to token NORENT
fn NORENT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NORENT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEQUENCE
/// Returns `None` if there is no child corresponding to token NOSEQUENCE
fn NOSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEQ
/// Returns `None` if there is no child corresponding to token NOSEQ
fn NOSEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSOURCE
/// Returns `None` if there is no child corresponding to token NOSOURCE
fn NOSOURCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSOURCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOS
/// Returns `None` if there is no child corresponding to token NOS
fn NOS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSPIE
/// Returns `None` if there is no child corresponding to token NOSPIE
fn NOSPIE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSPIE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQL
/// Returns `None` if there is no child corresponding to token NOSQL
fn NOSQL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQLCCSID
/// Returns `None` if there is no child corresponding to token NOSQLCCSID
fn NOSQLCCSID(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQLCCSID, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQLC
/// Returns `None` if there is no child corresponding to token NOSQLC
fn NOSQLC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQLC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSSRANGE
/// Returns `None` if there is no child corresponding to token NOSSRANGE
fn NOSSRANGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSSRANGE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSSR
/// Returns `None` if there is no child corresponding to token NOSSR
fn NOSSR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSSR, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSTDTRUNC
/// Returns `None` if there is no child corresponding to token NOSTDTRUNC
fn NOSTDTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSTDTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTERMINAL
/// Returns `None` if there is no child corresponding to token NOTERMINAL
fn NOTERMINAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTERMINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTERM
/// Returns `None` if there is no child corresponding to token NOTERM
fn NOTERM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTERM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTEST
/// Returns `None` if there is no child corresponding to token NOTEST
fn NOTEST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTEST, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTHREAD
/// Returns `None` if there is no child corresponding to token NOTHREAD
fn NOTHREAD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTHREAD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOVBREF
/// Returns `None` if there is no child corresponding to token NOVBREF
fn NOVBREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOVBREF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOWORD
/// Returns `None` if there is no child corresponding to token NOWORD
fn NOWORD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOWORD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOWD
/// Returns `None` if there is no child corresponding to token NOWD
fn NOWD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOWD, 0)
}
/// Retrieves first TerminalNode corresponding to token NSEQ
/// Returns `None` if there is no child corresponding to token NSEQ
fn NSEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NSEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NSYMBOL
/// Returns `None` if there is no child corresponding to token NSYMBOL
fn NSYMBOL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NSYMBOL, 0)
}
/// Retrieves first TerminalNode corresponding to token NS
/// Returns `None` if there is no child corresponding to token NS
fn NS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NS, 0)
}
/// Retrieves first TerminalNode corresponding to token NATIONAL
/// Returns `None` if there is no child corresponding to token NATIONAL
fn NATIONAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NATIONAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NAT
/// Returns `None` if there is no child corresponding to token NAT
fn NAT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NAT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOXREF
/// Returns `None` if there is no child corresponding to token NOXREF
fn NOXREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOXREF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOX
/// Returns `None` if there is no child corresponding to token NOX
fn NOX(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOX, 0)
}
/// Retrieves first TerminalNode corresponding to token NOZWB
/// Returns `None` if there is no child corresponding to token NOZWB
fn NOZWB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOZWB, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUM
/// Returns `None` if there is no child corresponding to token NUM
fn NUM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUM, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMPROC
/// Returns `None` if there is no child corresponding to token NUMPROC
fn NUMPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUMPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token MIG
/// Returns `None` if there is no child corresponding to token MIG
fn MIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MIG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOPFD
/// Returns `None` if there is no child corresponding to token NOPFD
fn NOPFD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOPFD, 0)
}
/// Retrieves first TerminalNode corresponding to token PFD
/// Returns `None` if there is no child corresponding to token PFD
fn PFD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PFD, 0)
}
/// Retrieves first TerminalNode corresponding to token OBJECT
/// Returns `None` if there is no child corresponding to token OBJECT
fn OBJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token OBJ
/// Returns `None` if there is no child corresponding to token OBJ
fn OBJ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OBJ, 0)
}
/// Retrieves first TerminalNode corresponding to token OFFSET
/// Returns `None` if there is no child corresponding to token OFFSET
fn OFFSET(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token OFF
/// Returns `None` if there is no child corresponding to token OFF
fn OFF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OFF, 0)
}
/// Retrieves first TerminalNode corresponding to token OPMARGINS
/// Returns `None` if there is no child corresponding to token OPMARGINS
fn OPMARGINS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPMARGINS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPSEQUENCE
/// Returns `None` if there is no child corresponding to token OPSEQUENCE
fn OPSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIMIZE
/// Returns `None` if there is no child corresponding to token OPTIMIZE
fn OPTIMIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTIMIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPT
/// Returns `None` if there is no child corresponding to token OPT
fn OPT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPT, 0)
}
/// Retrieves first TerminalNode corresponding to token FULL
/// Returns `None` if there is no child corresponding to token FULL
fn FULL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FULL, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTFILE
/// Returns `None` if there is no child corresponding to token OPTFILE
fn OPTFILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTFILE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONS
/// Returns `None` if there is no child corresponding to token OPTIONS
fn OPTIONS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTIONS, 0)
}
/// Retrieves first TerminalNode corresponding to token OP
/// Returns `None` if there is no child corresponding to token OP
fn OP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OP, 0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OUTDD
/// Returns `None` if there is no child corresponding to token OUTDD
fn OUTDD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OUTDD, 0)
}
/// Retrieves first TerminalNode corresponding to token OUT
/// Returns `None` if there is no child corresponding to token OUT
fn OUT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OUT, 0)
}
/// Retrieves first TerminalNode corresponding to token PGMNAME
/// Returns `None` if there is no child corresponding to token PGMNAME
fn PGMNAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PGMNAME, 0)
}
/// Retrieves first TerminalNode corresponding to token PGMN
/// Returns `None` if there is no child corresponding to token PGMN
fn PGMN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PGMN, 0)
}
/// Retrieves first TerminalNode corresponding to token CO
/// Returns `None` if there is no child corresponding to token CO
fn CO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CO, 0)
}
/// Retrieves first TerminalNode corresponding to token LM
/// Returns `None` if there is no child corresponding to token LM
fn LM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LM, 0)
}
/// Retrieves first TerminalNode corresponding to token LONGMIXED
/// Returns `None` if there is no child corresponding to token LONGMIXED
fn LONGMIXED(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LONGMIXED, 0)
}
/// Retrieves first TerminalNode corresponding to token LONGUPPER
/// Returns `None` if there is no child corresponding to token LONGUPPER
fn LONGUPPER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LONGUPPER, 0)
}
/// Retrieves first TerminalNode corresponding to token LU
/// Returns `None` if there is no child corresponding to token LU
fn LU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LU, 0)
}
/// Retrieves first TerminalNode corresponding to token MIXED
/// Returns `None` if there is no child corresponding to token MIXED
fn MIXED(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MIXED, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER
/// Returns `None` if there is no child corresponding to token UPPER
fn UPPER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(UPPER, 0)
}
/// Retrieves first TerminalNode corresponding to token PROLOG
/// Returns `None` if there is no child corresponding to token PROLOG
fn PROLOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PROLOG, 0)
}
/// Retrieves first TerminalNode corresponding to token QUOTE
/// Returns `None` if there is no child corresponding to token QUOTE
fn QUOTE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token Q_CHAR
/// Returns `None` if there is no child corresponding to token Q_CHAR
fn Q_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(Q_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RENT
/// Returns `None` if there is no child corresponding to token RENT
fn RENT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RENT, 0)
}
/// Retrieves first TerminalNode corresponding to token RMODE
/// Returns `None` if there is no child corresponding to token RMODE
fn RMODE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RMODE, 0)
}
/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token AUTO
/// Returns `None` if there is no child corresponding to token AUTO
fn AUTO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AUTO, 0)
}
/// Retrieves first TerminalNode corresponding to token SEQUENCE
/// Returns `None` if there is no child corresponding to token SEQUENCE
fn SEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token SEQ
/// Returns `None` if there is no child corresponding to token SEQ
fn SEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token SIZE
/// Returns `None` if there is no child corresponding to token SIZE
fn SIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token SZ
/// Returns `None` if there is no child corresponding to token SZ
fn SZ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SZ, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX
/// Returns `None` if there is no child corresponding to token MAX
fn MAX(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token SOURCE
/// Returns `None` if there is no child corresponding to token SOURCE
fn SOURCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SOURCE, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SP, 0)
}
/// Retrieves first TerminalNode corresponding to token SPACE
/// Returns `None` if there is no child corresponding to token SPACE
fn SPACE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SPACE, 0)
}
/// Retrieves first TerminalNode corresponding to token SPIE
/// Returns `None` if there is no child corresponding to token SPIE
fn SPIE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SPIE, 0)
}
/// Retrieves first TerminalNode corresponding to token SQL
/// Returns `None` if there is no child corresponding to token SQL
fn SQL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQL, 0)
}
/// Retrieves first TerminalNode corresponding to token SQLCCSID
/// Returns `None` if there is no child corresponding to token SQLCCSID
fn SQLCCSID(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQLCCSID, 0)
}
/// Retrieves first TerminalNode corresponding to token SQLC
/// Returns `None` if there is no child corresponding to token SQLC
fn SQLC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQLC, 0)
}
/// Retrieves first TerminalNode corresponding to token SSRANGE
/// Returns `None` if there is no child corresponding to token SSRANGE
fn SSRANGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SSRANGE, 0)
}
/// Retrieves first TerminalNode corresponding to token SSR
/// Returns `None` if there is no child corresponding to token SSR
fn SSR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SSR, 0)
}
/// Retrieves first TerminalNode corresponding to token SYSEIB
/// Returns `None` if there is no child corresponding to token SYSEIB
fn SYSEIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SYSEIB, 0)
}
/// Retrieves first TerminalNode corresponding to token TERMINAL
/// Returns `None` if there is no child corresponding to token TERMINAL
fn TERMINAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TERMINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TERM
/// Returns `None` if there is no child corresponding to token TERM
fn TERM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TERM, 0)
}
/// Retrieves first TerminalNode corresponding to token TEST
/// Returns `None` if there is no child corresponding to token TEST
fn TEST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TEST, 0)
}
/// Retrieves first TerminalNode corresponding to token HOOK
/// Returns `None` if there is no child corresponding to token HOOK
fn HOOK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(HOOK, 0)
}
/// Retrieves first TerminalNode corresponding to token NOHOOK
/// Returns `None` if there is no child corresponding to token NOHOOK
fn NOHOOK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOHOOK, 0)
}
/// Retrieves first TerminalNode corresponding to token SEP
/// Returns `None` if there is no child corresponding to token SEP
fn SEP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEP, 0)
}
/// Retrieves first TerminalNode corresponding to token SEPARATE
/// Returns `None` if there is no child corresponding to token SEPARATE
fn SEPARATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEPARATE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEP
/// Returns `None` if there is no child corresponding to token NOSEP
fn NOSEP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEPARATE
/// Returns `None` if there is no child corresponding to token NOSEPARATE
fn NOSEPARATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEPARATE, 0)
}
/// Retrieves first TerminalNode corresponding to token EJPD
/// Returns `None` if there is no child corresponding to token EJPD
fn EJPD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EJPD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEJPD
/// Returns `None` if there is no child corresponding to token NOEJPD
fn NOEJPD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEJPD, 0)
}
/// Retrieves first TerminalNode corresponding to token THREAD
/// Returns `None` if there is no child corresponding to token THREAD
fn THREAD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(THREAD, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUNC
/// Returns `None` if there is no child corresponding to token TRUNC
fn TRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token BIN
/// Returns `None` if there is no child corresponding to token BIN
fn BIN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BIN, 0)
}
/// Retrieves first TerminalNode corresponding to token VBREF
/// Returns `None` if there is no child corresponding to token VBREF
fn VBREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(VBREF, 0)
}
/// Retrieves first TerminalNode corresponding to token WORD
/// Returns `None` if there is no child corresponding to token WORD
fn WORD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(WORD, 0)
}
/// Retrieves first TerminalNode corresponding to token WD
/// Returns `None` if there is no child corresponding to token WD
fn WD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(WD, 0)
}
/// Retrieves first TerminalNode corresponding to token XMLPARSE
/// Returns `None` if there is no child corresponding to token XMLPARSE
fn XMLPARSE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XMLPARSE, 0)
}
/// Retrieves first TerminalNode corresponding to token XP
/// Returns `None` if there is no child corresponding to token XP
fn XP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XP, 0)
}
/// Retrieves first TerminalNode corresponding to token XMLSS
/// Returns `None` if there is no child corresponding to token XMLSS
fn XMLSS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XMLSS, 0)
}
/// Retrieves first TerminalNode corresponding to token X_CHAR
/// Returns `None` if there is no child corresponding to token X_CHAR
fn X_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(X_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token XREF
/// Returns `None` if there is no child corresponding to token XREF
fn XREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XREF, 0)
}
/// Retrieves first TerminalNode corresponding to token SHORT
/// Returns `None` if there is no child corresponding to token SHORT
fn SHORT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SHORT, 0)
}
/// Retrieves first TerminalNode corresponding to token YEARWINDOW
/// Returns `None` if there is no child corresponding to token YEARWINDOW
fn YEARWINDOW(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(YEARWINDOW, 0)
}
/// Retrieves first TerminalNode corresponding to token YW
/// Returns `None` if there is no child corresponding to token YW
fn YW(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(YW, 0)
}
/// Retrieves first TerminalNode corresponding to token ZWB
/// Returns `None` if there is no child corresponding to token ZWB
fn ZWB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ZWB, 0)
}

}

impl<'input> CompilerOptionContextAttrs<'input> for CompilerOptionContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilerOption(&mut self,)
	-> Result<Rc<CompilerOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilerOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_compilerOption);
        let mut _localctx: Rc<CompilerOptionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(445);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(103);
					recog.base.match_token(ADATA,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(104);
					recog.base.match_token(ADV,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(105);
					recog.base.match_token(APOST,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(106);
					_la = recog.base.input.la(1);
					if { !(_la==AR || _la==ARITH) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(107);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(108);
					_la = recog.base.input.la(1);
					if { !(_la==COMPAT || _la==EXTEND || _la==C_CHAR || _la==E_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(109);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(110);
					recog.base.match_token(AWO,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(111);
					recog.base.match_token(BLOCK0,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(112);
					_la = recog.base.input.la(1);
					if { !(_la==BUF || _la==BUFSIZE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(113);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(114);
					recog.literal()?;

					recog.base.set_state(115);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(117);
					recog.base.match_token(CBLCARD,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(118);
					recog.base.match_token(CICS,&mut recog.err_handler)?;

					recog.base.set_state(123);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(119);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							/*InvokeRule literal*/
							recog.base.set_state(120);
							recog.literal()?;

							recog.base.set_state(121);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(125);
					recog.base.match_token(COBOL2,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(126);
					recog.base.match_token(COBOL3,&mut recog.err_handler)?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					recog.base.set_state(127);
					_la = recog.base.input.la(1);
					if { !(_la==CODEPAGE || _la==CP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(128);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(129);
					recog.literal()?;

					recog.base.set_state(130);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					recog.base.set_state(132);
					_la = recog.base.input.la(1);
					if { !(_la==COMPILE || _la==C_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					recog.base.set_state(133);
					recog.base.match_token(CPP,&mut recog.err_handler)?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					recog.base.set_state(134);
					recog.base.match_token(CPSM,&mut recog.err_handler)?;

					}
				}
			,
				16 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					recog.base.set_state(135);
					_la = recog.base.input.la(1);
					if { !(_la==CURR || _la==CURRENCY) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(136);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(137);
					recog.literal()?;

					recog.base.set_state(138);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				17 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 17);
					recog.base.enter_outer_alt(None, 17);
					{
					recog.base.set_state(140);
					recog.base.match_token(DATA,&mut recog.err_handler)?;

					recog.base.set_state(141);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(142);
					recog.literal()?;

					recog.base.set_state(143);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				18 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 18);
					recog.base.enter_outer_alt(None, 18);
					{
					recog.base.set_state(145);
					_la = recog.base.input.la(1);
					if { !(_la==DATEPROC || _la==DP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(157);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(146);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(148);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==FLAG || _la==NOFLAG {
								{
								recog.base.set_state(147);
								_la = recog.base.input.la(1);
								if { !(_la==FLAG || _la==NOFLAG) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(151);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==COMMACHAR {
								{
								recog.base.set_state(150);
								recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(154);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==NOTRIG || _la==TRIG {
								{
								recog.base.set_state(153);
								_la = recog.base.input.la(1);
								if { !(_la==NOTRIG || _la==TRIG) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(156);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				19 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 19);
					recog.base.enter_outer_alt(None, 19);
					{
					recog.base.set_state(159);
					recog.base.match_token(DBCS,&mut recog.err_handler)?;

					}
				}
			,
				20 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 20);
					recog.base.enter_outer_alt(None, 20);
					{
					recog.base.set_state(160);
					_la = recog.base.input.la(1);
					if { !(_la==DECK || _la==D_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				21 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 21);
					recog.base.enter_outer_alt(None, 21);
					{
					recog.base.set_state(161);
					recog.base.match_token(DEBUG,&mut recog.err_handler)?;

					}
				}
			,
				22 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 22);
					recog.base.enter_outer_alt(None, 22);
					{
					recog.base.set_state(162);
					_la = recog.base.input.la(1);
					if { !(_la==DIAGTRUNC || _la==DTR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				23 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 23);
					recog.base.enter_outer_alt(None, 23);
					{
					recog.base.set_state(163);
					recog.base.match_token(DLL,&mut recog.err_handler)?;

					}
				}
			,
				24 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 24);
					recog.base.enter_outer_alt(None, 24);
					{
					recog.base.set_state(164);
					_la = recog.base.input.la(1);
					if { !(_la==DU || _la==DUMP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				25 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 25);
					recog.base.enter_outer_alt(None, 25);
					{
					recog.base.set_state(165);
					_la = recog.base.input.la(1);
					if { !(_la==DYN || _la==DYNAM) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				26 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 26);
					recog.base.enter_outer_alt(None, 26);
					{
					recog.base.set_state(166);
					recog.base.match_token(EDF,&mut recog.err_handler)?;

					}
				}
			,
				27 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 27);
					recog.base.enter_outer_alt(None, 27);
					{
					recog.base.set_state(167);
					recog.base.match_token(EPILOG,&mut recog.err_handler)?;

					}
				}
			,
				28 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 28);
					recog.base.enter_outer_alt(None, 28);
					{
					recog.base.set_state(168);
					recog.base.match_token(EXIT,&mut recog.err_handler)?;

					}
				}
			,
				29 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 29);
					recog.base.enter_outer_alt(None, 29);
					{
					recog.base.set_state(169);
					_la = recog.base.input.la(1);
					if { !(_la==EXP || _la==EXPORTALL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				30 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 30);
					recog.base.enter_outer_alt(None, 30);
					{
					recog.base.set_state(170);
					_la = recog.base.input.la(1);
					if { !(_la==FASTSRT || _la==FSRT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				31 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 31);
					recog.base.enter_outer_alt(None, 31);
					{
					recog.base.set_state(171);
					recog.base.match_token(FEPI,&mut recog.err_handler)?;

					}
				}
			,
				32 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 32);
					recog.base.enter_outer_alt(None, 32);
					{
					recog.base.set_state(172);
					_la = recog.base.input.la(1);
					if { !(_la==FLAG || _la==F_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(173);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(174);
					_la = recog.base.input.la(1);
					if { !(((((_la - 270)) & !0x3f) == 0 && ((1usize << (_la - 270)) & ((1usize << (E_CHAR - 270)) | (1usize << (I_CHAR - 270)) | (1usize << (S_CHAR - 270)) | (1usize << (U_CHAR - 270)) | (1usize << (W_CHAR - 270)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(177);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMACHAR {
						{
						recog.base.set_state(175);
						recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

						recog.base.set_state(176);
						_la = recog.base.input.la(1);
						if { !(((((_la - 270)) & !0x3f) == 0 && ((1usize << (_la - 270)) & ((1usize << (E_CHAR - 270)) | (1usize << (I_CHAR - 270)) | (1usize << (S_CHAR - 270)) | (1usize << (U_CHAR - 270)) | (1usize << (W_CHAR - 270)))) != 0)) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(179);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				33 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 33);
					recog.base.enter_outer_alt(None, 33);
					{
					recog.base.set_state(180);
					recog.base.match_token(FLAGSTD,&mut recog.err_handler)?;

					recog.base.set_state(181);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(182);
					_la = recog.base.input.la(1);
					if { !(((((_la - 272)) & !0x3f) == 0 && ((1usize << (_la - 272)) & ((1usize << (H_CHAR - 272)) | (1usize << (I_CHAR - 272)) | (1usize << (M_CHAR - 272)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(185);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMACHAR {
						{
						recog.base.set_state(183);
						recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

						recog.base.set_state(184);
						_la = recog.base.input.la(1);
						if { !(_la==DD || _la==NN || _la==SS || _la==D_CHAR || _la==N_CHAR || _la==S_CHAR) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(187);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				34 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 34);
					recog.base.enter_outer_alt(None, 34);
					{
					recog.base.set_state(188);
					recog.base.match_token(GDS,&mut recog.err_handler)?;

					}
				}
			,
				35 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 35);
					recog.base.enter_outer_alt(None, 35);
					{
					recog.base.set_state(189);
					recog.base.match_token(GRAPHIC,&mut recog.err_handler)?;

					}
				}
			,
				36 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 36);
					recog.base.enter_outer_alt(None, 36);
					{
					recog.base.set_state(190);
					recog.base.match_token(INTDATE,&mut recog.err_handler)?;

					recog.base.set_state(191);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(192);
					_la = recog.base.input.la(1);
					if { !(_la==ANSI || _la==LILIAN) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(193);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				37 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 37);
					recog.base.enter_outer_alt(None, 37);
					{
					recog.base.set_state(194);
					_la = recog.base.input.la(1);
					if { !(_la==LANG || _la==LANGUAGE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(195);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(196);
					_la = recog.base.input.la(1);
					if { !(_la==CS || ((((_la - 50)) & !0x3f) == 0 && ((1usize << (_la - 50)) & ((1usize << (EN - 50)) | (1usize << (ENGLISH - 50)) | (1usize << (JA - 50)) | (1usize << (JP - 50)) | (1usize << (KA - 50)))) != 0) || _la==UE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(197);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				38 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 38);
					recog.base.enter_outer_alt(None, 38);
					{
					recog.base.set_state(198);
					recog.base.match_token(LEASM,&mut recog.err_handler)?;

					}
				}
			,
				39 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 39);
					recog.base.enter_outer_alt(None, 39);
					{
					recog.base.set_state(199);
					recog.base.match_token(LENGTH,&mut recog.err_handler)?;

					}
				}
			,
				40 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 40);
					recog.base.enter_outer_alt(None, 40);
					{
					recog.base.set_state(200);
					recog.base.match_token(LIB,&mut recog.err_handler)?;

					}
				}
			,
				41 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 41);
					recog.base.enter_outer_alt(None, 41);
					{
					recog.base.set_state(201);
					recog.base.match_token(LIN,&mut recog.err_handler)?;

					}
				}
			,
				42 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 42);
					recog.base.enter_outer_alt(None, 42);
					{
					recog.base.set_state(202);
					_la = recog.base.input.la(1);
					if { !(_la==LC || _la==LINECOUNT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(203);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(204);
					recog.literal()?;

					recog.base.set_state(205);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				43 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 43);
					recog.base.enter_outer_alt(None, 43);
					{
					recog.base.set_state(207);
					recog.base.match_token(LINKAGE,&mut recog.err_handler)?;

					}
				}
			,
				44 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 44);
					recog.base.enter_outer_alt(None, 44);
					{
					recog.base.set_state(208);
					recog.base.match_token(LIST,&mut recog.err_handler)?;

					}
				}
			,
				45 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 45);
					recog.base.enter_outer_alt(None, 45);
					{
					recog.base.set_state(209);
					recog.base.match_token(MAP,&mut recog.err_handler)?;

					}
				}
			,
				46 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 46);
					recog.base.enter_outer_alt(None, 46);
					{
					recog.base.set_state(210);
					recog.base.match_token(MARGINS,&mut recog.err_handler)?;

					recog.base.set_state(211);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(212);
					recog.literal()?;

					recog.base.set_state(213);
					recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(214);
					recog.literal()?;

					recog.base.set_state(217);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMACHAR {
						{
						recog.base.set_state(215);
						recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

						/*InvokeRule literal*/
						recog.base.set_state(216);
						recog.literal()?;

						}
					}

					recog.base.set_state(219);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				47 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 47);
					recog.base.enter_outer_alt(None, 47);
					{
					recog.base.set_state(221);
					_la = recog.base.input.la(1);
					if { !(_la==MD || _la==MDECK) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(225);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(222);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(223);
							_la = recog.base.input.la(1);
							if { !(_la==COMPILE || _la==NOC || _la==NOCOMPILE || _la==C_CHAR) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							recog.base.set_state(224);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				48 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 48);
					recog.base.enter_outer_alt(None, 48);
					{
					recog.base.set_state(227);
					recog.base.match_token(NAME,&mut recog.err_handler)?;

					recog.base.set_state(231);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(228);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(229);
							_la = recog.base.input.la(1);
							if { !(_la==ALIAS || _la==NOALIAS) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							recog.base.set_state(230);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				49 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 49);
					recog.base.enter_outer_alt(None, 49);
					{
					recog.base.set_state(233);
					recog.base.match_token(NATLANG,&mut recog.err_handler)?;

					recog.base.set_state(234);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(235);
					_la = recog.base.input.la(1);
					if { !(_la==CS || _la==EN || _la==KA) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(236);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				50 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 50);
					recog.base.enter_outer_alt(None, 50);
					{
					recog.base.set_state(237);
					recog.base.match_token(NOADATA,&mut recog.err_handler)?;

					}
				}
			,
				51 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 51);
					recog.base.enter_outer_alt(None, 51);
					{
					recog.base.set_state(238);
					recog.base.match_token(NOADV,&mut recog.err_handler)?;

					}
				}
			,
				52 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 52);
					recog.base.enter_outer_alt(None, 52);
					{
					recog.base.set_state(239);
					recog.base.match_token(NOAWO,&mut recog.err_handler)?;

					}
				}
			,
				53 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 53);
					recog.base.enter_outer_alt(None, 53);
					{
					recog.base.set_state(240);
					recog.base.match_token(NOBLOCK0,&mut recog.err_handler)?;

					}
				}
			,
				54 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 54);
					recog.base.enter_outer_alt(None, 54);
					{
					recog.base.set_state(241);
					recog.base.match_token(NOCBLCARD,&mut recog.err_handler)?;

					}
				}
			,
				55 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 55);
					recog.base.enter_outer_alt(None, 55);
					{
					recog.base.set_state(242);
					recog.base.match_token(NOCICS,&mut recog.err_handler)?;

					}
				}
			,
				56 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 56);
					recog.base.enter_outer_alt(None, 56);
					{
					recog.base.set_state(243);
					recog.base.match_token(NOCMPR2,&mut recog.err_handler)?;

					}
				}
			,
				57 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 57);
					recog.base.enter_outer_alt(None, 57);
					{
					recog.base.set_state(244);
					_la = recog.base.input.la(1);
					if { !(_la==NOC || _la==NOCOMPILE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(248);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(245);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(246);
							_la = recog.base.input.la(1);
							if { !(((((_la - 270)) & !0x3f) == 0 && ((1usize << (_la - 270)) & ((1usize << (E_CHAR - 270)) | (1usize << (S_CHAR - 270)) | (1usize << (W_CHAR - 270)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							recog.base.set_state(247);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				58 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 58);
					recog.base.enter_outer_alt(None, 58);
					{
					recog.base.set_state(250);
					recog.base.match_token(NOCPSM,&mut recog.err_handler)?;

					}
				}
			,
				59 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 59);
					recog.base.enter_outer_alt(None, 59);
					{
					recog.base.set_state(251);
					_la = recog.base.input.la(1);
					if { !(_la==NOCURR || _la==NOCURRENCY) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				60 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 60);
					recog.base.enter_outer_alt(None, 60);
					{
					recog.base.set_state(252);
					_la = recog.base.input.la(1);
					if { !(_la==NODATEPROC || _la==NODP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				61 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 61);
					recog.base.enter_outer_alt(None, 61);
					{
					recog.base.set_state(253);
					recog.base.match_token(NODBCS,&mut recog.err_handler)?;

					}
				}
			,
				62 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 62);
					recog.base.enter_outer_alt(None, 62);
					{
					recog.base.set_state(254);
					recog.base.match_token(NODEBUG,&mut recog.err_handler)?;

					}
				}
			,
				63 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 63);
					recog.base.enter_outer_alt(None, 63);
					{
					recog.base.set_state(255);
					_la = recog.base.input.la(1);
					if { !(_la==NOD || _la==NODECK) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				64 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 64);
					recog.base.enter_outer_alt(None, 64);
					{
					recog.base.set_state(256);
					recog.base.match_token(NODLL,&mut recog.err_handler)?;

					}
				}
			,
				65 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 65);
					recog.base.enter_outer_alt(None, 65);
					{
					recog.base.set_state(257);
					recog.base.match_token(NODE,&mut recog.err_handler)?;

					}
				}
			,
				66 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 66);
					recog.base.enter_outer_alt(None, 66);
					{
					recog.base.set_state(258);
					_la = recog.base.input.la(1);
					if { !(_la==NODU || _la==NODUMP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				67 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 67);
					recog.base.enter_outer_alt(None, 67);
					{
					recog.base.set_state(259);
					_la = recog.base.input.la(1);
					if { !(_la==NODIAGTRUNC || _la==NODTR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				68 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 68);
					recog.base.enter_outer_alt(None, 68);
					{
					recog.base.set_state(260);
					_la = recog.base.input.la(1);
					if { !(_la==NODYN || _la==NODYNAM) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				69 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 69);
					recog.base.enter_outer_alt(None, 69);
					{
					recog.base.set_state(261);
					recog.base.match_token(NOEDF,&mut recog.err_handler)?;

					}
				}
			,
				70 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 70);
					recog.base.enter_outer_alt(None, 70);
					{
					recog.base.set_state(262);
					recog.base.match_token(NOEPILOG,&mut recog.err_handler)?;

					}
				}
			,
				71 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 71);
					recog.base.enter_outer_alt(None, 71);
					{
					recog.base.set_state(263);
					recog.base.match_token(NOEXIT,&mut recog.err_handler)?;

					}
				}
			,
				72 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 72);
					recog.base.enter_outer_alt(None, 72);
					{
					recog.base.set_state(264);
					_la = recog.base.input.la(1);
					if { !(_la==NOEXP || _la==NOEXPORTALL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				73 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 73);
					recog.base.enter_outer_alt(None, 73);
					{
					recog.base.set_state(265);
					_la = recog.base.input.la(1);
					if { !(_la==NOFASTSRT || _la==NOFSRT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				74 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 74);
					recog.base.enter_outer_alt(None, 74);
					{
					recog.base.set_state(266);
					recog.base.match_token(NOFEPI,&mut recog.err_handler)?;

					}
				}
			,
				75 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 75);
					recog.base.enter_outer_alt(None, 75);
					{
					recog.base.set_state(267);
					_la = recog.base.input.la(1);
					if { !(_la==NOF || _la==NOFLAG) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				76 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 76);
					recog.base.enter_outer_alt(None, 76);
					{
					recog.base.set_state(268);
					recog.base.match_token(NOFLAGMIG,&mut recog.err_handler)?;

					}
				}
			,
				77 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 77);
					recog.base.enter_outer_alt(None, 77);
					{
					recog.base.set_state(269);
					recog.base.match_token(NOFLAGSTD,&mut recog.err_handler)?;

					}
				}
			,
				78 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 78);
					recog.base.enter_outer_alt(None, 78);
					{
					recog.base.set_state(270);
					recog.base.match_token(NOGRAPHIC,&mut recog.err_handler)?;

					}
				}
			,
				79 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 79);
					recog.base.enter_outer_alt(None, 79);
					{
					recog.base.set_state(271);
					recog.base.match_token(NOLENGTH,&mut recog.err_handler)?;

					}
				}
			,
				80 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 80);
					recog.base.enter_outer_alt(None, 80);
					{
					recog.base.set_state(272);
					recog.base.match_token(NOLIB,&mut recog.err_handler)?;

					}
				}
			,
				81 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 81);
					recog.base.enter_outer_alt(None, 81);
					{
					recog.base.set_state(273);
					recog.base.match_token(NOLINKAGE,&mut recog.err_handler)?;

					}
				}
			,
				82 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 82);
					recog.base.enter_outer_alt(None, 82);
					{
					recog.base.set_state(274);
					recog.base.match_token(NOLIST,&mut recog.err_handler)?;

					}
				}
			,
				83 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 83);
					recog.base.enter_outer_alt(None, 83);
					{
					recog.base.set_state(275);
					recog.base.match_token(NOMAP,&mut recog.err_handler)?;

					}
				}
			,
				84 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 84);
					recog.base.enter_outer_alt(None, 84);
					{
					recog.base.set_state(276);
					_la = recog.base.input.la(1);
					if { !(_la==NOMD || _la==NOMDECK) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				85 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 85);
					recog.base.enter_outer_alt(None, 85);
					{
					recog.base.set_state(277);
					recog.base.match_token(NONAME,&mut recog.err_handler)?;

					}
				}
			,
				86 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 86);
					recog.base.enter_outer_alt(None, 86);
					{
					recog.base.set_state(278);
					_la = recog.base.input.la(1);
					if { !(_la==NONUM || _la==NONUMBER) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				87 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 87);
					recog.base.enter_outer_alt(None, 87);
					{
					recog.base.set_state(279);
					_la = recog.base.input.la(1);
					if { !(_la==NOOBJ || _la==NOOBJECT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				88 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 88);
					recog.base.enter_outer_alt(None, 88);
					{
					recog.base.set_state(280);
					_la = recog.base.input.la(1);
					if { !(_la==NOOFF || _la==NOOFFSET) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				89 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 89);
					recog.base.enter_outer_alt(None, 89);
					{
					recog.base.set_state(281);
					recog.base.match_token(NOOPSEQUENCE,&mut recog.err_handler)?;

					}
				}
			,
				90 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 90);
					recog.base.enter_outer_alt(None, 90);
					{
					recog.base.set_state(282);
					_la = recog.base.input.la(1);
					if { !(_la==NOOPT || _la==NOOPTIMIZE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				91 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 91);
					recog.base.enter_outer_alt(None, 91);
					{
					recog.base.set_state(283);
					recog.base.match_token(NOOPTIONS,&mut recog.err_handler)?;

					}
				}
			,
				92 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 92);
					recog.base.enter_outer_alt(None, 92);
					{
					recog.base.set_state(284);
					recog.base.match_token(NOP,&mut recog.err_handler)?;

					}
				}
			,
				93 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 93);
					recog.base.enter_outer_alt(None, 93);
					{
					recog.base.set_state(285);
					recog.base.match_token(NOPROLOG,&mut recog.err_handler)?;

					}
				}
			,
				94 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 94);
					recog.base.enter_outer_alt(None, 94);
					{
					recog.base.set_state(286);
					recog.base.match_token(NORENT,&mut recog.err_handler)?;

					}
				}
			,
				95 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 95);
					recog.base.enter_outer_alt(None, 95);
					{
					recog.base.set_state(287);
					_la = recog.base.input.la(1);
					if { !(_la==NOSEQ || _la==NOSEQUENCE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				96 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 96);
					recog.base.enter_outer_alt(None, 96);
					{
					recog.base.set_state(288);
					_la = recog.base.input.la(1);
					if { !(_la==NOS || _la==NOSOURCE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				97 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 97);
					recog.base.enter_outer_alt(None, 97);
					{
					recog.base.set_state(289);
					recog.base.match_token(NOSPIE,&mut recog.err_handler)?;

					}
				}
			,
				98 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 98);
					recog.base.enter_outer_alt(None, 98);
					{
					recog.base.set_state(290);
					recog.base.match_token(NOSQL,&mut recog.err_handler)?;

					}
				}
			,
				99 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 99);
					recog.base.enter_outer_alt(None, 99);
					{
					recog.base.set_state(291);
					_la = recog.base.input.la(1);
					if { !(_la==NOSQLC || _la==NOSQLCCSID) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				100 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 100);
					recog.base.enter_outer_alt(None, 100);
					{
					recog.base.set_state(292);
					_la = recog.base.input.la(1);
					if { !(_la==NOSSR || _la==NOSSRANGE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				101 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 101);
					recog.base.enter_outer_alt(None, 101);
					{
					recog.base.set_state(293);
					recog.base.match_token(NOSTDTRUNC,&mut recog.err_handler)?;

					}
				}
			,
				102 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 102);
					recog.base.enter_outer_alt(None, 102);
					{
					recog.base.set_state(294);
					_la = recog.base.input.la(1);
					if { !(_la==NOTERM || _la==NOTERMINAL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				103 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 103);
					recog.base.enter_outer_alt(None, 103);
					{
					recog.base.set_state(295);
					recog.base.match_token(NOTEST,&mut recog.err_handler)?;

					}
				}
			,
				104 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 104);
					recog.base.enter_outer_alt(None, 104);
					{
					recog.base.set_state(296);
					recog.base.match_token(NOTHREAD,&mut recog.err_handler)?;

					}
				}
			,
				105 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 105);
					recog.base.enter_outer_alt(None, 105);
					{
					recog.base.set_state(297);
					recog.base.match_token(NOVBREF,&mut recog.err_handler)?;

					}
				}
			,
				106 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 106);
					recog.base.enter_outer_alt(None, 106);
					{
					recog.base.set_state(298);
					_la = recog.base.input.la(1);
					if { !(_la==NOWD || _la==NOWORD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				107 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 107);
					recog.base.enter_outer_alt(None, 107);
					{
					recog.base.set_state(299);
					recog.base.match_token(NSEQ,&mut recog.err_handler)?;

					}
				}
			,
				108 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 108);
					recog.base.enter_outer_alt(None, 108);
					{
					recog.base.set_state(300);
					_la = recog.base.input.la(1);
					if { !(_la==NS || _la==NSYMBOL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(301);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(302);
					_la = recog.base.input.la(1);
					if { !(_la==DBCS || _la==NAT || _la==NATIONAL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(303);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				109 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 109);
					recog.base.enter_outer_alt(None, 109);
					{
					recog.base.set_state(304);
					recog.base.match_token(NOVBREF,&mut recog.err_handler)?;

					}
				}
			,
				110 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 110);
					recog.base.enter_outer_alt(None, 110);
					{
					recog.base.set_state(305);
					_la = recog.base.input.la(1);
					if { !(_la==NOX || _la==NOXREF) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				111 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 111);
					recog.base.enter_outer_alt(None, 111);
					{
					recog.base.set_state(306);
					recog.base.match_token(NOZWB,&mut recog.err_handler)?;

					}
				}
			,
				112 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 112);
					recog.base.enter_outer_alt(None, 112);
					{
					recog.base.set_state(307);
					_la = recog.base.input.la(1);
					if { !(_la==NUM || _la==NUMBER) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				113 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 113);
					recog.base.enter_outer_alt(None, 113);
					{
					recog.base.set_state(308);
					recog.base.match_token(NUMPROC,&mut recog.err_handler)?;

					recog.base.set_state(309);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(310);
					_la = recog.base.input.la(1);
					if { !(_la==MIG || _la==NOPFD || _la==PFD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(311);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				114 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 114);
					recog.base.enter_outer_alt(None, 114);
					{
					recog.base.set_state(312);
					_la = recog.base.input.la(1);
					if { !(_la==OBJ || _la==OBJECT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				115 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 115);
					recog.base.enter_outer_alt(None, 115);
					{
					recog.base.set_state(313);
					_la = recog.base.input.la(1);
					if { !(_la==OFF || _la==OFFSET) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				116 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 116);
					recog.base.enter_outer_alt(None, 116);
					{
					recog.base.set_state(314);
					recog.base.match_token(OPMARGINS,&mut recog.err_handler)?;

					recog.base.set_state(315);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(316);
					recog.literal()?;

					recog.base.set_state(317);
					recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(318);
					recog.literal()?;

					recog.base.set_state(321);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMACHAR {
						{
						recog.base.set_state(319);
						recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

						/*InvokeRule literal*/
						recog.base.set_state(320);
						recog.literal()?;

						}
					}

					recog.base.set_state(323);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				117 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 117);
					recog.base.enter_outer_alt(None, 117);
					{
					recog.base.set_state(325);
					recog.base.match_token(OPSEQUENCE,&mut recog.err_handler)?;

					recog.base.set_state(326);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(327);
					recog.literal()?;

					recog.base.set_state(328);
					recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(329);
					recog.literal()?;

					recog.base.set_state(330);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				118 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 118);
					recog.base.enter_outer_alt(None, 118);
					{
					recog.base.set_state(332);
					_la = recog.base.input.la(1);
					if { !(_la==OPT || _la==OPTIMIZE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(336);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(333);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(334);
							_la = recog.base.input.la(1);
							if { !(_la==FULL || _la==STD) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							recog.base.set_state(335);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				119 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 119);
					recog.base.enter_outer_alt(None, 119);
					{
					recog.base.set_state(338);
					recog.base.match_token(OPTFILE,&mut recog.err_handler)?;

					}
				}
			,
				120 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 120);
					recog.base.enter_outer_alt(None, 120);
					{
					recog.base.set_state(339);
					recog.base.match_token(OPTIONS,&mut recog.err_handler)?;

					}
				}
			,
				121 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 121);
					recog.base.enter_outer_alt(None, 121);
					{
					recog.base.set_state(340);
					recog.base.match_token(OP,&mut recog.err_handler)?;

					}
				}
			,
				122 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 122);
					recog.base.enter_outer_alt(None, 122);
					{
					recog.base.set_state(341);
					_la = recog.base.input.la(1);
					if { !(_la==OUT || _la==OUTDD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(342);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule cobolWord*/
					recog.base.set_state(343);
					recog.cobolWord()?;

					recog.base.set_state(344);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				123 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 123);
					recog.base.enter_outer_alt(None, 123);
					{
					recog.base.set_state(346);
					_la = recog.base.input.la(1);
					if { !(_la==PGMN || _la==PGMNAME) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(347);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(348);
					_la = recog.base.input.la(1);
					if { !(_la==CO || _la==COMPAT || ((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (LM - 85)) | (1usize << (LONGMIXED - 85)) | (1usize << (LONGUPPER - 85)) | (1usize << (LU - 85)) | (1usize << (MIXED - 85)))) != 0) || ((((_la - 256)) & !0x3f) == 0 && ((1usize << (_la - 256)) & ((1usize << (UPPER - 256)) | (1usize << (M_CHAR - 256)) | (1usize << (U_CHAR - 256)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(349);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				124 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 124);
					recog.base.enter_outer_alt(None, 124);
					{
					recog.base.set_state(350);
					recog.base.match_token(PROLOG,&mut recog.err_handler)?;

					}
				}
			,
				125 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 125);
					recog.base.enter_outer_alt(None, 125);
					{
					recog.base.set_state(351);
					_la = recog.base.input.la(1);
					if { !(_la==QUOTE || _la==Q_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				126 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 126);
					recog.base.enter_outer_alt(None, 126);
					{
					recog.base.set_state(352);
					recog.base.match_token(RENT,&mut recog.err_handler)?;

					}
				}
			,
				127 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 127);
					recog.base.enter_outer_alt(None, 127);
					{
					recog.base.set_state(353);
					recog.base.match_token(RMODE,&mut recog.err_handler)?;

					recog.base.set_state(354);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(358);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 ANY 
						=> {
							{
							recog.base.set_state(355);
							recog.base.match_token(ANY,&mut recog.err_handler)?;

							}
						}

					 AUTO 
						=> {
							{
							recog.base.set_state(356);
							recog.base.match_token(AUTO,&mut recog.err_handler)?;

							}
						}

					 NONNUMERICLITERAL | NUMERICLITERAL 
						=> {
							{
							/*InvokeRule literal*/
							recog.base.set_state(357);
							recog.literal()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(360);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				128 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 128);
					recog.base.enter_outer_alt(None, 128);
					{
					recog.base.set_state(361);
					_la = recog.base.input.la(1);
					if { !(_la==SEQ || _la==SEQUENCE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(368);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(362);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							/*InvokeRule literal*/
							recog.base.set_state(363);
							recog.literal()?;

							recog.base.set_state(364);
							recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

							/*InvokeRule literal*/
							recog.base.set_state(365);
							recog.literal()?;

							recog.base.set_state(366);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				129 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 129);
					recog.base.enter_outer_alt(None, 129);
					{
					recog.base.set_state(370);
					_la = recog.base.input.la(1);
					if { !(_la==SIZE || _la==SZ) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(371);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(374);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 MAX 
						=> {
							{
							recog.base.set_state(372);
							recog.base.match_token(MAX,&mut recog.err_handler)?;

							}
						}

					 NONNUMERICLITERAL | NUMERICLITERAL 
						=> {
							{
							/*InvokeRule literal*/
							recog.base.set_state(373);
							recog.literal()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(376);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				130 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 130);
					recog.base.enter_outer_alt(None, 130);
					{
					recog.base.set_state(377);
					_la = recog.base.input.la(1);
					if { !(_la==SOURCE || _la==S_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				131 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 131);
					recog.base.enter_outer_alt(None, 131);
					{
					recog.base.set_state(378);
					recog.base.match_token(SP,&mut recog.err_handler)?;

					}
				}
			,
				132 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 132);
					recog.base.enter_outer_alt(None, 132);
					{
					recog.base.set_state(379);
					recog.base.match_token(SPACE,&mut recog.err_handler)?;

					recog.base.set_state(380);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(381);
					recog.literal()?;

					recog.base.set_state(382);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				133 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 133);
					recog.base.enter_outer_alt(None, 133);
					{
					recog.base.set_state(384);
					recog.base.match_token(SPIE,&mut recog.err_handler)?;

					}
				}
			,
				134 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 134);
					recog.base.enter_outer_alt(None, 134);
					{
					recog.base.set_state(385);
					recog.base.match_token(SQL,&mut recog.err_handler)?;

					recog.base.set_state(390);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(386);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							/*InvokeRule literal*/
							recog.base.set_state(387);
							recog.literal()?;

							recog.base.set_state(388);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				135 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 135);
					recog.base.enter_outer_alt(None, 135);
					{
					recog.base.set_state(392);
					_la = recog.base.input.la(1);
					if { !(_la==SQLC || _la==SQLCCSID) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				136 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 136);
					recog.base.enter_outer_alt(None, 136);
					{
					recog.base.set_state(393);
					_la = recog.base.input.la(1);
					if { !(_la==SSR || _la==SSRANGE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				137 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 137);
					recog.base.enter_outer_alt(None, 137);
					{
					recog.base.set_state(394);
					recog.base.match_token(SYSEIB,&mut recog.err_handler)?;

					}
				}
			,
				138 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 138);
					recog.base.enter_outer_alt(None, 138);
					{
					recog.base.set_state(395);
					_la = recog.base.input.la(1);
					if { !(_la==TERM || _la==TERMINAL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				139 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 139);
					recog.base.enter_outer_alt(None, 139);
					{
					recog.base.set_state(396);
					recog.base.match_token(TEST,&mut recog.err_handler)?;

					recog.base.set_state(414);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(397);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(399);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==HOOK || _la==NOHOOK {
								{
								recog.base.set_state(398);
								_la = recog.base.input.la(1);
								if { !(_la==HOOK || _la==NOHOOK) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(402);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(401);
									recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							recog.base.set_state(405);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==NOSEP || _la==NOSEPARATE || _la==SEP || _la==SEPARATE {
								{
								recog.base.set_state(404);
								_la = recog.base.input.la(1);
								if { !(_la==NOSEP || _la==NOSEPARATE || _la==SEP || _la==SEPARATE) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(408);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==COMMACHAR {
								{
								recog.base.set_state(407);
								recog.base.match_token(COMMACHAR,&mut recog.err_handler)?;

								}
							}

							recog.base.set_state(411);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==EJPD || _la==NOEJPD {
								{
								recog.base.set_state(410);
								_la = recog.base.input.la(1);
								if { !(_la==EJPD || _la==NOEJPD) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(413);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				140 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 140);
					recog.base.enter_outer_alt(None, 140);
					{
					recog.base.set_state(416);
					recog.base.match_token(THREAD,&mut recog.err_handler)?;

					}
				}
			,
				141 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 141);
					recog.base.enter_outer_alt(None, 141);
					{
					recog.base.set_state(417);
					recog.base.match_token(TRUNC,&mut recog.err_handler)?;

					recog.base.set_state(418);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(419);
					_la = recog.base.input.la(1);
					if { !(_la==BIN || _la==OPT || _la==STD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(420);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				142 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 142);
					recog.base.enter_outer_alt(None, 142);
					{
					recog.base.set_state(421);
					recog.base.match_token(VBREF,&mut recog.err_handler)?;

					}
				}
			,
				143 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 143);
					recog.base.enter_outer_alt(None, 143);
					{
					recog.base.set_state(422);
					_la = recog.base.input.la(1);
					if { !(_la==WD || _la==WORD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(423);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule cobolWord*/
					recog.base.set_state(424);
					recog.cobolWord()?;

					recog.base.set_state(425);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				144 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 144);
					recog.base.enter_outer_alt(None, 144);
					{
					recog.base.set_state(427);
					_la = recog.base.input.la(1);
					if { !(_la==XMLPARSE || _la==XP) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(428);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					recog.base.set_state(429);
					_la = recog.base.input.la(1);
					if { !(_la==COMPAT || ((((_la - 261)) & !0x3f) == 0 && ((1usize << (_la - 261)) & ((1usize << (XMLSS - 261)) | (1usize << (C_CHAR - 261)) | (1usize << (X_CHAR - 261)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(430);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				145 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 145);
					recog.base.enter_outer_alt(None, 145);
					{
					recog.base.set_state(431);
					_la = recog.base.input.la(1);
					if { !(_la==XREF || _la==X_CHAR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(437);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(432);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							recog.base.set_state(434);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==FULL || _la==SHORT {
								{
								recog.base.set_state(433);
								_la = recog.base.input.la(1);
								if { !(_la==FULL || _la==SHORT) } {
									recog.err_handler.recover_inline(&mut recog.base)?;

								}
								else {
									if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
									recog.err_handler.report_match(&mut recog.base);
									recog.base.consume(&mut recog.err_handler);
								}
								}
							}

							recog.base.set_state(436);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				146 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 146);
					recog.base.enter_outer_alt(None, 146);
					{
					recog.base.set_state(439);
					_la = recog.base.input.la(1);
					if { !(_la==YEARWINDOW || _la==YW) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(440);
					recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(441);
					recog.literal()?;

					recog.base.set_state(442);
					recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

					}
				}
			,
				147 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 147);
					recog.base.enter_outer_alt(None, 147);
					{
					recog.base.set_state(444);
					recog.base.match_token(ZWB,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- execCicsStatement ----------------
pub type ExecCicsStatementContextAll<'input> = ExecCicsStatementContext<'input>;


pub type ExecCicsStatementContext<'input> = BaseParserRuleContext<'input,ExecCicsStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExecCicsStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ExecCicsStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ExecCicsStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_execCicsStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_execCicsStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ExecCicsStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_execCicsStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExecCicsStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_execCicsStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_execCicsStatement }
}
antlr_rust::tid!{ExecCicsStatementContextExt<'a>}

impl<'input> ExecCicsStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExecCicsStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExecCicsStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExecCicsStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ExecCicsStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXEC
/// Returns `None` if there is no child corresponding to token EXEC
fn EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token CICS
/// Returns `None` if there is no child corresponding to token CICS
fn CICS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CICS, 0)
}
fn charData(&self) -> Option<Rc<CharDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END_EXEC
/// Returns `None` if there is no child corresponding to token END_EXEC
fn END_EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(END_EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ExecCicsStatementContextAttrs<'input> for ExecCicsStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn execCicsStatement(&mut self,)
	-> Result<Rc<ExecCicsStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExecCicsStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_execCicsStatement);
        let mut _localctx: Rc<ExecCicsStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(447);
			recog.base.match_token(EXEC,&mut recog.err_handler)?;

			recog.base.set_state(448);
			recog.base.match_token(CICS,&mut recog.err_handler)?;

			/*InvokeRule charData*/
			recog.base.set_state(449);
			recog.charData()?;

			recog.base.set_state(450);
			recog.base.match_token(END_EXEC,&mut recog.err_handler)?;

			recog.base.set_state(452);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(451);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- execSqlStatement ----------------
pub type ExecSqlStatementContextAll<'input> = ExecSqlStatementContext<'input>;


pub type ExecSqlStatementContext<'input> = BaseParserRuleContext<'input,ExecSqlStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExecSqlStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ExecSqlStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ExecSqlStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_execSqlStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_execSqlStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ExecSqlStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_execSqlStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExecSqlStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_execSqlStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_execSqlStatement }
}
antlr_rust::tid!{ExecSqlStatementContextExt<'a>}

impl<'input> ExecSqlStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExecSqlStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExecSqlStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExecSqlStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ExecSqlStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXEC
/// Returns `None` if there is no child corresponding to token EXEC
fn EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token SQL
/// Returns `None` if there is no child corresponding to token SQL
fn SQL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQL, 0)
}
fn charDataSql(&self) -> Option<Rc<CharDataSqlContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END_EXEC
/// Returns `None` if there is no child corresponding to token END_EXEC
fn END_EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(END_EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ExecSqlStatementContextAttrs<'input> for ExecSqlStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn execSqlStatement(&mut self,)
	-> Result<Rc<ExecSqlStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExecSqlStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_execSqlStatement);
        let mut _localctx: Rc<ExecSqlStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(454);
			recog.base.match_token(EXEC,&mut recog.err_handler)?;

			recog.base.set_state(455);
			recog.base.match_token(SQL,&mut recog.err_handler)?;

			/*InvokeRule charDataSql*/
			recog.base.set_state(456);
			recog.charDataSql()?;

			recog.base.set_state(457);
			recog.base.match_token(END_EXEC,&mut recog.err_handler)?;

			recog.base.set_state(459);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(458);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- execSqlImsStatement ----------------
pub type ExecSqlImsStatementContextAll<'input> = ExecSqlImsStatementContext<'input>;


pub type ExecSqlImsStatementContext<'input> = BaseParserRuleContext<'input,ExecSqlImsStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExecSqlImsStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ExecSqlImsStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ExecSqlImsStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_execSqlImsStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_execSqlImsStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ExecSqlImsStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_execSqlImsStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExecSqlImsStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_execSqlImsStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_execSqlImsStatement }
}
antlr_rust::tid!{ExecSqlImsStatementContextExt<'a>}

impl<'input> ExecSqlImsStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExecSqlImsStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExecSqlImsStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExecSqlImsStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ExecSqlImsStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXEC
/// Returns `None` if there is no child corresponding to token EXEC
fn EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token SQLIMS
/// Returns `None` if there is no child corresponding to token SQLIMS
fn SQLIMS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQLIMS, 0)
}
fn charData(&self) -> Option<Rc<CharDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END_EXEC
/// Returns `None` if there is no child corresponding to token END_EXEC
fn END_EXEC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(END_EXEC, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ExecSqlImsStatementContextAttrs<'input> for ExecSqlImsStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn execSqlImsStatement(&mut self,)
	-> Result<Rc<ExecSqlImsStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExecSqlImsStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_execSqlImsStatement);
        let mut _localctx: Rc<ExecSqlImsStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(461);
			recog.base.match_token(EXEC,&mut recog.err_handler)?;

			recog.base.set_state(462);
			recog.base.match_token(SQLIMS,&mut recog.err_handler)?;

			/*InvokeRule charData*/
			recog.base.set_state(463);
			recog.charData()?;

			recog.base.set_state(464);
			recog.base.match_token(END_EXEC,&mut recog.err_handler)?;

			recog.base.set_state(466);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(465);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- copyStatement ----------------
pub type CopyStatementContextAll<'input> = CopyStatementContext<'input>;


pub type CopyStatementContext<'input> = BaseParserRuleContext<'input,CopyStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CopyStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CopyStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CopyStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_copyStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_copyStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CopyStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_copyStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CopyStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_copyStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_copyStatement }
}
antlr_rust::tid!{CopyStatementContextExt<'a>}

impl<'input> CopyStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CopyStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CopyStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CopyStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CopyStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COPY
/// Returns `None` if there is no child corresponding to token COPY
fn COPY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COPY, 0)
}
fn copySource(&self) -> Option<Rc<CopySourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
fn directoryPhrase_all(&self) ->  Vec<Rc<DirectoryPhraseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn directoryPhrase(&self, i: usize) -> Option<Rc<DirectoryPhraseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn familyPhrase_all(&self) ->  Vec<Rc<FamilyPhraseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn familyPhrase(&self, i: usize) -> Option<Rc<FamilyPhraseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn replacingPhrase_all(&self) ->  Vec<Rc<ReplacingPhraseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn replacingPhrase(&self, i: usize) -> Option<Rc<ReplacingPhraseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SUPPRESS in current rule
fn SUPPRESS_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SUPPRESS, starting from 0.
/// Returns `None` if number of children corresponding to token SUPPRESS is less or equal than `i`.
fn SUPPRESS(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SUPPRESS, i)
}

}

impl<'input> CopyStatementContextAttrs<'input> for CopyStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn copyStatement(&mut self,)
	-> Result<Rc<CopyStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CopyStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_copyStatement);
        let mut _localctx: Rc<CopyStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(468);
			recog.base.match_token(COPY,&mut recog.err_handler)?;

			/*InvokeRule copySource*/
			recog.base.set_state(469);
			recog.copySource()?;

			recog.base.set_state(484);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(473);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(470);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(475);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(480);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 IN | OF 
						=> {
							{
							/*InvokeRule directoryPhrase*/
							recog.base.set_state(476);
							recog.directoryPhrase()?;

							}
						}

					 ON 
						=> {
							{
							/*InvokeRule familyPhrase*/
							recog.base.set_state(477);
							recog.familyPhrase()?;

							}
						}

					 REPLACING 
						=> {
							{
							/*InvokeRule replacingPhrase*/
							recog.base.set_state(478);
							recog.replacingPhrase()?;

							}
						}

					 SUPPRESS 
						=> {
							{
							recog.base.set_state(479);
							recog.base.match_token(SUPPRESS,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
					} 
				}
				recog.base.set_state(486);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			}
			recog.base.set_state(490);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(487);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(492);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(493);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- copySource ----------------
pub type CopySourceContextAll<'input> = CopySourceContext<'input>;


pub type CopySourceContext<'input> = BaseParserRuleContext<'input,CopySourceContextExt<'input>>;

#[derive(Clone)]
pub struct CopySourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CopySourceContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CopySourceContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_copySource(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_copySource(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CopySourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_copySource(self);
	}
}

impl<'input> CustomRuleContext<'input> for CopySourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_copySource }
	//fn type_rule_index() -> usize where Self: Sized { RULE_copySource }
}
antlr_rust::tid!{CopySourceContextExt<'a>}

impl<'input> CopySourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CopySourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CopySourceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CopySourceContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CopySourceContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn filename(&self) -> Option<Rc<FilenameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn copyLibrary(&self) -> Option<Rc<CopyLibraryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OF, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}

}

impl<'input> CopySourceContextAttrs<'input> for CopySourceContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn copySource(&mut self,)
	-> Result<Rc<CopySourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CopySourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_copySource);
        let mut _localctx: Rc<CopySourceContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NONNUMERICLITERAL | NUMERICLITERAL 
				=> {
					{
					/*InvokeRule literal*/
					recog.base.set_state(495);
					recog.literal()?;

					}
				}

			 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
			 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
			 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
			 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
			 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
			 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
			 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
			 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE | LIST |
			 LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS | MAX | MD | MDECK |
			 MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN | NO | NOADATA | NOADV |
			 NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
			 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
			 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
			 NODYNAM | NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL |
			 NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT |
			 NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP |
			 NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF |
			 NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD |
			 NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE |
			 NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE |
			 NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD |
			 NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC |
			 OBJ | OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE |
			 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
			 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | SEP |
			 SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE | SPIE |
			 SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB | SZ | TERM |
			 TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE | UPPER | VBREF |
			 WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW | YW | ZWB | C_CHAR |
			 D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR |
			 S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | IDENTIFIER 
				=> {
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(496);
					recog.cobolWord()?;

					}
				}

			 FILENAME 
				=> {
					{
					/*InvokeRule filename*/
					recog.base.set_state(497);
					recog.filename()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(502);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(41,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(500);
					_la = recog.base.input.la(1);
					if { !(_la==IN || _la==OF) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule copyLibrary*/
					recog.base.set_state(501);
					recog.copyLibrary()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- copyLibrary ----------------
pub type CopyLibraryContextAll<'input> = CopyLibraryContext<'input>;


pub type CopyLibraryContext<'input> = BaseParserRuleContext<'input,CopyLibraryContextExt<'input>>;

#[derive(Clone)]
pub struct CopyLibraryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CopyLibraryContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CopyLibraryContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_copyLibrary(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_copyLibrary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CopyLibraryContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_copyLibrary(self);
	}
}

impl<'input> CustomRuleContext<'input> for CopyLibraryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_copyLibrary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_copyLibrary }
}
antlr_rust::tid!{CopyLibraryContextExt<'a>}

impl<'input> CopyLibraryContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CopyLibraryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CopyLibraryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CopyLibraryContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CopyLibraryContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CopyLibraryContextAttrs<'input> for CopyLibraryContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn copyLibrary(&mut self,)
	-> Result<Rc<CopyLibraryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CopyLibraryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_copyLibrary);
        let mut _localctx: Rc<CopyLibraryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(506);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NONNUMERICLITERAL | NUMERICLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(504);
					recog.literal()?;

					}
				}

			 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
			 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
			 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
			 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
			 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
			 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
			 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
			 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE | LIST |
			 LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS | MAX | MD | MDECK |
			 MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN | NO | NOADATA | NOADV |
			 NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
			 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
			 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
			 NODYNAM | NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL |
			 NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT |
			 NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP |
			 NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF |
			 NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD |
			 NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE |
			 NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE |
			 NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD |
			 NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC |
			 OBJ | OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE |
			 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
			 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | SEP |
			 SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE | SPIE |
			 SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB | SZ | TERM |
			 TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE | UPPER | VBREF |
			 WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW | YW | ZWB | C_CHAR |
			 D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR |
			 S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(505);
					recog.cobolWord()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replacingPhrase ----------------
pub type ReplacingPhraseContextAll<'input> = ReplacingPhraseContext<'input>;


pub type ReplacingPhraseContext<'input> = BaseParserRuleContext<'input,ReplacingPhraseContextExt<'input>>;

#[derive(Clone)]
pub struct ReplacingPhraseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplacingPhraseContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplacingPhraseContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replacingPhrase(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replacingPhrase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplacingPhraseContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replacingPhrase(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplacingPhraseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replacingPhrase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replacingPhrase }
}
antlr_rust::tid!{ReplacingPhraseContextExt<'a>}

impl<'input> ReplacingPhraseContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplacingPhraseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplacingPhraseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplacingPhraseContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplacingPhraseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REPLACING
/// Returns `None` if there is no child corresponding to token REPLACING
fn REPLACING(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(REPLACING, 0)
}
fn replaceClause_all(&self) ->  Vec<Rc<ReplaceClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn replaceClause(&self, i: usize) -> Option<Rc<ReplaceClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> ReplacingPhraseContextAttrs<'input> for ReplacingPhraseContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replacingPhrase(&mut self,)
	-> Result<Rc<ReplacingPhraseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplacingPhraseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_replacingPhrase);
        let mut _localctx: Rc<ReplacingPhraseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(508);
			recog.base.match_token(REPLACING,&mut recog.err_handler)?;

			recog.base.set_state(512);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(509);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(514);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule replaceClause*/
			recog.base.set_state(515);
			recog.replaceClause()?;

			recog.base.set_state(524);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(45,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(517); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(516);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(519); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==NEWLINE) {break}
					}
					/*InvokeRule replaceClause*/
					recog.base.set_state(521);
					recog.replaceClause()?;

					}
					} 
				}
				recog.base.set_state(526);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(45,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replaceArea ----------------
pub type ReplaceAreaContextAll<'input> = ReplaceAreaContext<'input>;


pub type ReplaceAreaContext<'input> = BaseParserRuleContext<'input,ReplaceAreaContextExt<'input>>;

#[derive(Clone)]
pub struct ReplaceAreaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplaceAreaContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplaceAreaContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replaceArea(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replaceArea(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplaceAreaContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replaceArea(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplaceAreaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replaceArea }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replaceArea }
}
antlr_rust::tid!{ReplaceAreaContextExt<'a>}

impl<'input> ReplaceAreaContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplaceAreaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplaceAreaContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplaceAreaContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplaceAreaContextExt<'input>>{

fn replaceByStatement(&self) -> Option<Rc<ReplaceByStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn copyStatement_all(&self) ->  Vec<Rc<CopyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn copyStatement(&self, i: usize) -> Option<Rc<CopyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn charData_all(&self) ->  Vec<Rc<CharDataContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn charData(&self, i: usize) -> Option<Rc<CharDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn replaceOffStatement(&self) -> Option<Rc<ReplaceOffStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReplaceAreaContextAttrs<'input> for ReplaceAreaContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replaceArea(&mut self,)
	-> Result<Rc<ReplaceAreaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplaceAreaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_replaceArea);
        let mut _localctx: Rc<ReplaceAreaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule replaceByStatement*/
			recog.base.set_state(527);
			recog.replaceByStatement()?;

			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(47,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(530);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 COPY 
						=> {
							{
							/*InvokeRule copyStatement*/
							recog.base.set_state(528);
							recog.copyStatement()?;

							}
						}

					 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
					 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
					 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
					 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
					 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
					 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG |
					 FLAGSTD | FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA |
					 JP | KA | LANG | LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT |
					 LINKAGE | LIST | LM | LONGMIXED | LONGUPPER | LPARENCHAR | LU | MAP |
					 MARGINS | MAX | MD | MDECK | MIG | MIXED | NAME | NAT | NATIONAL |
					 NATLANG | NN | NO | NOADATA | NOADV | NOALIAS | NOAWO | NOBLOCK0 |
					 NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE | NOCPSM | NOCURR |
					 NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG | NODECK |
					 NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN | NODYNAM |
					 NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL | NOF | NOFASTSRT |
					 NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT | NOGRAPHIC | NOHOOK |
					 NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP | NOMD | NOMDECK | NONAME |
					 NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF | NOOFFSET | NOOPSEQUENCE |
					 NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD | NOPROLOG | NORENT |
					 NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE | NOSQL | NOSQLC |
					 NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE | NOTERM |
					 NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD | NOX |
					 NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC | OBJ |
					 OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE | OPT |
					 OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
					 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | RPARENCHAR |
					 SEP | SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE |
					 SPIE | SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB |
					 SZ | TERM | TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE |
					 UPPER | VBREF | WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW |
					 YW | ZWB | C_CHAR | D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR |
					 N_CHAR | Q_CHAR | S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | DOT |
					 NONNUMERICLITERAL | NUMERICLITERAL | IDENTIFIER | FILENAME | NEWLINE |
					 TEXT 
						=> {
							{
							/*InvokeRule charData*/
							recog.base.set_state(529);
							recog.charData()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					} 
				}
				recog.base.set_state(534);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(47,&mut recog.base)?;
			}
			recog.base.set_state(536);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule replaceOffStatement*/
					recog.base.set_state(535);
					recog.replaceOffStatement()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replaceByStatement ----------------
pub type ReplaceByStatementContextAll<'input> = ReplaceByStatementContext<'input>;


pub type ReplaceByStatementContext<'input> = BaseParserRuleContext<'input,ReplaceByStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ReplaceByStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplaceByStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplaceByStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replaceByStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replaceByStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplaceByStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replaceByStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplaceByStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replaceByStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replaceByStatement }
}
antlr_rust::tid!{ReplaceByStatementContextExt<'a>}

impl<'input> ReplaceByStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplaceByStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplaceByStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplaceByStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplaceByStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REPLACE
/// Returns `None` if there is no child corresponding to token REPLACE
fn REPLACE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(REPLACE, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn replaceClause_all(&self) ->  Vec<Rc<ReplaceClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn replaceClause(&self, i: usize) -> Option<Rc<ReplaceClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> ReplaceByStatementContextAttrs<'input> for ReplaceByStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replaceByStatement(&mut self,)
	-> Result<Rc<ReplaceByStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplaceByStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_replaceByStatement);
        let mut _localctx: Rc<ReplaceByStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(538);
			recog.base.match_token(REPLACE,&mut recog.err_handler)?;

			recog.base.set_state(546); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(542);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(539);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(544);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule replaceClause*/
					recog.base.set_state(545);
					recog.replaceClause()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(548); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(50,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			recog.base.set_state(550);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replaceOffStatement ----------------
pub type ReplaceOffStatementContextAll<'input> = ReplaceOffStatementContext<'input>;


pub type ReplaceOffStatementContext<'input> = BaseParserRuleContext<'input,ReplaceOffStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ReplaceOffStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplaceOffStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplaceOffStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replaceOffStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replaceOffStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplaceOffStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replaceOffStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplaceOffStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replaceOffStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replaceOffStatement }
}
antlr_rust::tid!{ReplaceOffStatementContextExt<'a>}

impl<'input> ReplaceOffStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplaceOffStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplaceOffStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplaceOffStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplaceOffStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REPLACE
/// Returns `None` if there is no child corresponding to token REPLACE
fn REPLACE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(REPLACE, 0)
}
/// Retrieves first TerminalNode corresponding to token OFF
/// Returns `None` if there is no child corresponding to token OFF
fn OFF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OFF, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> ReplaceOffStatementContextAttrs<'input> for ReplaceOffStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replaceOffStatement(&mut self,)
	-> Result<Rc<ReplaceOffStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplaceOffStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_replaceOffStatement);
        let mut _localctx: Rc<ReplaceOffStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(552);
			recog.base.match_token(REPLACE,&mut recog.err_handler)?;

			recog.base.set_state(553);
			recog.base.match_token(OFF,&mut recog.err_handler)?;

			recog.base.set_state(554);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replaceClause ----------------
pub type ReplaceClauseContextAll<'input> = ReplaceClauseContext<'input>;


pub type ReplaceClauseContext<'input> = BaseParserRuleContext<'input,ReplaceClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ReplaceClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplaceClauseContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplaceClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replaceClause(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replaceClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplaceClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replaceClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplaceClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replaceClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replaceClause }
}
antlr_rust::tid!{ReplaceClauseContextExt<'a>}

impl<'input> ReplaceClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplaceClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplaceClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplaceClauseContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplaceClauseContextExt<'input>>{

fn replaceable(&self) -> Option<Rc<ReplaceableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BY, 0)
}
fn replacement(&self) -> Option<Rc<ReplacementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
fn directoryPhrase(&self) -> Option<Rc<DirectoryPhraseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn familyPhrase(&self) -> Option<Rc<FamilyPhraseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReplaceClauseContextAttrs<'input> for ReplaceClauseContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replaceClause(&mut self,)
	-> Result<Rc<ReplaceClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplaceClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_replaceClause);
        let mut _localctx: Rc<ReplaceClauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule replaceable*/
			recog.base.set_state(556);
			recog.replaceable()?;

			recog.base.set_state(560);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(557);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(562);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(563);
			recog.base.match_token(BY,&mut recog.err_handler)?;

			recog.base.set_state(567);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(564);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(569);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule replacement*/
			recog.base.set_state(570);
			recog.replacement()?;

			recog.base.set_state(578);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(54,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(574);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(571);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(576);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule directoryPhrase*/
					recog.base.set_state(577);
					recog.directoryPhrase()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(587);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(583);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(580);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(585);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule familyPhrase*/
					recog.base.set_state(586);
					recog.familyPhrase()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- directoryPhrase ----------------
pub type DirectoryPhraseContextAll<'input> = DirectoryPhraseContext<'input>;


pub type DirectoryPhraseContext<'input> = BaseParserRuleContext<'input,DirectoryPhraseContextExt<'input>>;

#[derive(Clone)]
pub struct DirectoryPhraseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for DirectoryPhraseContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for DirectoryPhraseContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_directoryPhrase(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_directoryPhrase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for DirectoryPhraseContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_directoryPhrase(self);
	}
}

impl<'input> CustomRuleContext<'input> for DirectoryPhraseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directoryPhrase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directoryPhrase }
}
antlr_rust::tid!{DirectoryPhraseContextExt<'a>}

impl<'input> DirectoryPhraseContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DirectoryPhraseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectoryPhraseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DirectoryPhraseContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<DirectoryPhraseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OF, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> DirectoryPhraseContextAttrs<'input> for DirectoryPhraseContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn directoryPhrase(&mut self,)
	-> Result<Rc<DirectoryPhraseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DirectoryPhraseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_directoryPhrase);
        let mut _localctx: Rc<DirectoryPhraseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(589);
			_la = recog.base.input.la(1);
			if { !(_la==IN || _la==OF) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(593);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(590);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(595);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(598);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NONNUMERICLITERAL | NUMERICLITERAL 
				=> {
					{
					/*InvokeRule literal*/
					recog.base.set_state(596);
					recog.literal()?;

					}
				}

			 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
			 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
			 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
			 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
			 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
			 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
			 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
			 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE | LIST |
			 LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS | MAX | MD | MDECK |
			 MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN | NO | NOADATA | NOADV |
			 NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
			 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
			 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
			 NODYNAM | NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL |
			 NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT |
			 NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP |
			 NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF |
			 NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD |
			 NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE |
			 NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE |
			 NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD |
			 NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC |
			 OBJ | OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE |
			 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
			 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | SEP |
			 SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE | SPIE |
			 SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB | SZ | TERM |
			 TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE | UPPER | VBREF |
			 WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW | YW | ZWB | C_CHAR |
			 D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR |
			 S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | IDENTIFIER 
				=> {
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(597);
					recog.cobolWord()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- familyPhrase ----------------
pub type FamilyPhraseContextAll<'input> = FamilyPhraseContext<'input>;


pub type FamilyPhraseContext<'input> = BaseParserRuleContext<'input,FamilyPhraseContextExt<'input>>;

#[derive(Clone)]
pub struct FamilyPhraseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for FamilyPhraseContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for FamilyPhraseContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_familyPhrase(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_familyPhrase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for FamilyPhraseContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_familyPhrase(self);
	}
}

impl<'input> CustomRuleContext<'input> for FamilyPhraseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_familyPhrase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_familyPhrase }
}
antlr_rust::tid!{FamilyPhraseContextExt<'a>}

impl<'input> FamilyPhraseContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FamilyPhraseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FamilyPhraseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FamilyPhraseContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<FamilyPhraseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ON
/// Returns `None` if there is no child corresponding to token ON
fn ON(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ON, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> FamilyPhraseContextAttrs<'input> for FamilyPhraseContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn familyPhrase(&mut self,)
	-> Result<Rc<FamilyPhraseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FamilyPhraseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_familyPhrase);
        let mut _localctx: Rc<FamilyPhraseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(600);
			recog.base.match_token(ON,&mut recog.err_handler)?;

			recog.base.set_state(604);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(601);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(606);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(609);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NONNUMERICLITERAL | NUMERICLITERAL 
				=> {
					{
					/*InvokeRule literal*/
					recog.base.set_state(607);
					recog.literal()?;

					}
				}

			 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
			 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
			 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
			 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
			 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
			 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
			 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
			 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE | LIST |
			 LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS | MAX | MD | MDECK |
			 MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN | NO | NOADATA | NOADV |
			 NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
			 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
			 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
			 NODYNAM | NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL |
			 NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT |
			 NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP |
			 NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF |
			 NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD |
			 NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE |
			 NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE |
			 NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD |
			 NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC |
			 OBJ | OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE |
			 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
			 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | SEP |
			 SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE | SPIE |
			 SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB | SZ | TERM |
			 TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE | UPPER | VBREF |
			 WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW | YW | ZWB | C_CHAR |
			 D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR |
			 S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | IDENTIFIER 
				=> {
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(608);
					recog.cobolWord()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replaceable ----------------
pub type ReplaceableContextAll<'input> = ReplaceableContext<'input>;


pub type ReplaceableContext<'input> = BaseParserRuleContext<'input,ReplaceableContextExt<'input>>;

#[derive(Clone)]
pub struct ReplaceableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplaceableContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplaceableContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replaceable(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replaceable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplaceableContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replaceable(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplaceableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replaceable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replaceable }
}
antlr_rust::tid!{ReplaceableContextExt<'a>}

impl<'input> ReplaceableContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplaceableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplaceableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplaceableContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplaceableContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pseudoText(&self) -> Option<Rc<PseudoTextContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn charDataLine(&self) -> Option<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReplaceableContextAttrs<'input> for ReplaceableContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replaceable(&mut self,)
	-> Result<Rc<ReplaceableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplaceableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_replaceable);
        let mut _localctx: Rc<ReplaceableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(615);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(611);
					recog.literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(612);
					recog.cobolWord()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule pseudoText*/
					recog.base.set_state(613);
					recog.pseudoText()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule charDataLine*/
					recog.base.set_state(614);
					recog.charDataLine()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- replacement ----------------
pub type ReplacementContextAll<'input> = ReplacementContext<'input>;


pub type ReplacementContext<'input> = BaseParserRuleContext<'input,ReplacementContextExt<'input>>;

#[derive(Clone)]
pub struct ReplacementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for ReplacementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for ReplacementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_replacement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_replacement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for ReplacementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_replacement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReplacementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_replacement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_replacement }
}
antlr_rust::tid!{ReplacementContextExt<'a>}

impl<'input> ReplacementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReplacementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReplacementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReplacementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<ReplacementContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cobolWord(&self) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pseudoText(&self) -> Option<Rc<PseudoTextContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn charDataLine(&self) -> Option<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReplacementContextAttrs<'input> for ReplacementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn replacement(&mut self,)
	-> Result<Rc<ReplacementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReplacementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_replacement);
        let mut _localctx: Rc<ReplacementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(621);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(617);
					recog.literal()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule cobolWord*/
					recog.base.set_state(618);
					recog.cobolWord()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule pseudoText*/
					recog.base.set_state(619);
					recog.pseudoText()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule charDataLine*/
					recog.base.set_state(620);
					recog.charDataLine()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ejectStatement ----------------
pub type EjectStatementContextAll<'input> = EjectStatementContext<'input>;


pub type EjectStatementContext<'input> = BaseParserRuleContext<'input,EjectStatementContextExt<'input>>;

#[derive(Clone)]
pub struct EjectStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for EjectStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for EjectStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ejectStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_ejectStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for EjectStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_ejectStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EjectStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ejectStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ejectStatement }
}
antlr_rust::tid!{EjectStatementContextExt<'a>}

impl<'input> EjectStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EjectStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EjectStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EjectStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<EjectStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EJECT
/// Returns `None` if there is no child corresponding to token EJECT
fn EJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> EjectStatementContextAttrs<'input> for EjectStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ejectStatement(&mut self,)
	-> Result<Rc<EjectStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EjectStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_ejectStatement);
        let mut _localctx: Rc<EjectStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(623);
			recog.base.match_token(EJECT,&mut recog.err_handler)?;

			recog.base.set_state(625);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(624);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- skipStatement ----------------
pub type SkipStatementContextAll<'input> = SkipStatementContext<'input>;


pub type SkipStatementContext<'input> = BaseParserRuleContext<'input,SkipStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SkipStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for SkipStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for SkipStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_skipStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_skipStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for SkipStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_skipStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for SkipStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_skipStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_skipStatement }
}
antlr_rust::tid!{SkipStatementContextExt<'a>}

impl<'input> SkipStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SkipStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SkipStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SkipStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<SkipStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SKIP1
/// Returns `None` if there is no child corresponding to token SKIP1
fn SKIP1(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SKIP1, 0)
}
/// Retrieves first TerminalNode corresponding to token SKIP2
/// Returns `None` if there is no child corresponding to token SKIP2
fn SKIP2(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SKIP2, 0)
}
/// Retrieves first TerminalNode corresponding to token SKIP3
/// Returns `None` if there is no child corresponding to token SKIP3
fn SKIP3(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SKIP3, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> SkipStatementContextAttrs<'input> for SkipStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn skipStatement(&mut self,)
	-> Result<Rc<SkipStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SkipStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_skipStatement);
        let mut _localctx: Rc<SkipStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(627);
			_la = recog.base.input.la(1);
			if { !(((((_la - 238)) & !0x3f) == 0 && ((1usize << (_la - 238)) & ((1usize << (SKIP1 - 238)) | (1usize << (SKIP2 - 238)) | (1usize << (SKIP3 - 238)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(629);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(628);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- titleStatement ----------------
pub type TitleStatementContextAll<'input> = TitleStatementContext<'input>;


pub type TitleStatementContext<'input> = BaseParserRuleContext<'input,TitleStatementContextExt<'input>>;

#[derive(Clone)]
pub struct TitleStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for TitleStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for TitleStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_titleStatement(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_titleStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for TitleStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_titleStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for TitleStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_titleStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_titleStatement }
}
antlr_rust::tid!{TitleStatementContextExt<'a>}

impl<'input> TitleStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TitleStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TitleStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TitleStatementContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<TitleStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TITLE
/// Returns `None` if there is no child corresponding to token TITLE
fn TITLE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TITLE, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> TitleStatementContextAttrs<'input> for TitleStatementContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn titleStatement(&mut self,)
	-> Result<Rc<TitleStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TitleStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_titleStatement);
        let mut _localctx: Rc<TitleStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(631);
			recog.base.match_token(TITLE,&mut recog.err_handler)?;

			/*InvokeRule literal*/
			recog.base.set_state(632);
			recog.literal()?;

			recog.base.set_state(634);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(633);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pseudoText ----------------
pub type PseudoTextContextAll<'input> = PseudoTextContext<'input>;


pub type PseudoTextContext<'input> = BaseParserRuleContext<'input,PseudoTextContextExt<'input>>;

#[derive(Clone)]
pub struct PseudoTextContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for PseudoTextContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for PseudoTextContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pseudoText(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_pseudoText(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for PseudoTextContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_pseudoText(self);
	}
}

impl<'input> CustomRuleContext<'input> for PseudoTextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pseudoText }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pseudoText }
}
antlr_rust::tid!{PseudoTextContextExt<'a>}

impl<'input> PseudoTextContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PseudoTextContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PseudoTextContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PseudoTextContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<PseudoTextContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token DOUBLEEQUALCHAR in current rule
fn DOUBLEEQUALCHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOUBLEEQUALCHAR, starting from 0.
/// Returns `None` if number of children corresponding to token DOUBLEEQUALCHAR is less or equal than `i`.
fn DOUBLEEQUALCHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOUBLEEQUALCHAR, i)
}
fn charData(&self) -> Option<Rc<CharDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PseudoTextContextAttrs<'input> for PseudoTextContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pseudoText(&mut self,)
	-> Result<Rc<PseudoTextContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PseudoTextContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_pseudoText);
        let mut _localctx: Rc<PseudoTextContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(636);
			recog.base.match_token(DOUBLEEQUALCHAR,&mut recog.err_handler)?;

			recog.base.set_state(638);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADATA) | (1usize << ADV) | (1usize << ALIAS) | (1usize << ANSI) | (1usize << ANY) | (1usize << APOST) | (1usize << AR) | (1usize << ARITH) | (1usize << AUTO) | (1usize << AWO) | (1usize << BIN) | (1usize << BLOCK0) | (1usize << BUF) | (1usize << BUFSIZE) | (1usize << BY) | (1usize << CBL) | (1usize << CBLCARD) | (1usize << CO) | (1usize << COBOL2) | (1usize << COBOL3) | (1usize << CODEPAGE) | (1usize << COMPAT) | (1usize << COMPILE) | (1usize << CP) | (1usize << CPP) | (1usize << CPSM) | (1usize << CS) | (1usize << CURR) | (1usize << CURRENCY))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (DATA - 32)) | (1usize << (DATEPROC - 32)) | (1usize << (DBCS - 32)) | (1usize << (DD - 32)) | (1usize << (DEBUG - 32)) | (1usize << (DECK - 32)) | (1usize << (DIAGTRUNC - 32)) | (1usize << (DLI - 32)) | (1usize << (DLL - 32)) | (1usize << (DP - 32)) | (1usize << (DTR - 32)) | (1usize << (DU - 32)) | (1usize << (DUMP - 32)) | (1usize << (DYN - 32)) | (1usize << (DYNAM - 32)) | (1usize << (EDF - 32)) | (1usize << (EJECT - 32)) | (1usize << (EJPD - 32)) | (1usize << (EN - 32)) | (1usize << (ENGLISH - 32)) | (1usize << (EPILOG - 32)) | (1usize << (EXCI - 32)) | (1usize << (EXIT - 32)) | (1usize << (EXP - 32)) | (1usize << (EXPORTALL - 32)) | (1usize << (EXTEND - 32)) | (1usize << (FASTSRT - 32)) | (1usize << (FLAG - 32)) | (1usize << (FLAGSTD - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (FSRT - 64)) | (1usize << (FULL - 64)) | (1usize << (GDS - 64)) | (1usize << (GRAPHIC - 64)) | (1usize << (HOOK - 64)) | (1usize << (IN - 64)) | (1usize << (INTDATE - 64)) | (1usize << (JA - 64)) | (1usize << (JP - 64)) | (1usize << (KA - 64)) | (1usize << (LANG - 64)) | (1usize << (LANGUAGE - 64)) | (1usize << (LC - 64)) | (1usize << (LENGTH - 64)) | (1usize << (LIB - 64)) | (1usize << (LILIAN - 64)) | (1usize << (LIN - 64)) | (1usize << (LINECOUNT - 64)) | (1usize << (LINKAGE - 64)) | (1usize << (LIST - 64)) | (1usize << (LM - 64)) | (1usize << (LONGMIXED - 64)) | (1usize << (LONGUPPER - 64)) | (1usize << (LPARENCHAR - 64)) | (1usize << (LU - 64)) | (1usize << (MAP - 64)) | (1usize << (MARGINS - 64)) | (1usize << (MAX - 64)) | (1usize << (MD - 64)) | (1usize << (MDECK - 64)) | (1usize << (MIG - 64)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MIXED - 96)) | (1usize << (NAME - 96)) | (1usize << (NAT - 96)) | (1usize << (NATIONAL - 96)) | (1usize << (NATLANG - 96)) | (1usize << (NN - 96)) | (1usize << (NO - 96)) | (1usize << (NOADATA - 96)) | (1usize << (NOADV - 96)) | (1usize << (NOALIAS - 96)) | (1usize << (NOAWO - 96)) | (1usize << (NOBLOCK0 - 96)) | (1usize << (NOC - 96)) | (1usize << (NOCBLCARD - 96)) | (1usize << (NOCICS - 96)) | (1usize << (NOCMPR2 - 96)) | (1usize << (NOCOMPILE - 96)) | (1usize << (NOCPSM - 96)) | (1usize << (NOCURR - 96)) | (1usize << (NOCURRENCY - 96)) | (1usize << (NOD - 96)) | (1usize << (NODATEPROC - 96)) | (1usize << (NODBCS - 96)) | (1usize << (NODE - 96)) | (1usize << (NODEBUG - 96)) | (1usize << (NODECK - 96)) | (1usize << (NODIAGTRUNC - 96)) | (1usize << (NODLL - 96)) | (1usize << (NODU - 96)) | (1usize << (NODUMP - 96)) | (1usize << (NODP - 96)) | (1usize << (NODTR - 96)))) != 0) || ((((_la - 128)) & !0x3f) == 0 && ((1usize << (_la - 128)) & ((1usize << (NODYN - 128)) | (1usize << (NODYNAM - 128)) | (1usize << (NOEDF - 128)) | (1usize << (NOEJPD - 128)) | (1usize << (NOEPILOG - 128)) | (1usize << (NOEXIT - 128)) | (1usize << (NOEXP - 128)) | (1usize << (NOEXPORTALL - 128)) | (1usize << (NOF - 128)) | (1usize << (NOFASTSRT - 128)) | (1usize << (NOFEPI - 128)) | (1usize << (NOFLAG - 128)) | (1usize << (NOFLAGMIG - 128)) | (1usize << (NOFLAGSTD - 128)) | (1usize << (NOFSRT - 128)) | (1usize << (NOGRAPHIC - 128)) | (1usize << (NOHOOK - 128)) | (1usize << (NOLENGTH - 128)) | (1usize << (NOLIB - 128)) | (1usize << (NOLINKAGE - 128)) | (1usize << (NOLIST - 128)) | (1usize << (NOMAP - 128)) | (1usize << (NOMD - 128)) | (1usize << (NOMDECK - 128)) | (1usize << (NONAME - 128)) | (1usize << (NONUM - 128)) | (1usize << (NONUMBER - 128)) | (1usize << (NOOBJ - 128)) | (1usize << (NOOBJECT - 128)) | (1usize << (NOOFF - 128)) | (1usize << (NOOFFSET - 128)) | (1usize << (NOOPSEQUENCE - 128)))) != 0) || ((((_la - 160)) & !0x3f) == 0 && ((1usize << (_la - 160)) & ((1usize << (NOOPT - 160)) | (1usize << (NOOPTIMIZE - 160)) | (1usize << (NOOPTIONS - 160)) | (1usize << (NOP - 160)) | (1usize << (NOPFD - 160)) | (1usize << (NOPROLOG - 160)) | (1usize << (NORENT - 160)) | (1usize << (NOS - 160)) | (1usize << (NOSEP - 160)) | (1usize << (NOSEPARATE - 160)) | (1usize << (NOSEQ - 160)) | (1usize << (NOSOURCE - 160)) | (1usize << (NOSPIE - 160)) | (1usize << (NOSQL - 160)) | (1usize << (NOSQLC - 160)) | (1usize << (NOSQLCCSID - 160)) | (1usize << (NOSSR - 160)) | (1usize << (NOSSRANGE - 160)) | (1usize << (NOSTDTRUNC - 160)) | (1usize << (NOSEQUENCE - 160)) | (1usize << (NOTERM - 160)) | (1usize << (NOTERMINAL - 160)) | (1usize << (NOTEST - 160)) | (1usize << (NOTHREAD - 160)) | (1usize << (NOTRIG - 160)) | (1usize << (NOVBREF - 160)) | (1usize << (NOWORD - 160)) | (1usize << (NOX - 160)) | (1usize << (NOXREF - 160)) | (1usize << (NOZWB - 160)) | (1usize << (NS - 160)))) != 0) || ((((_la - 192)) & !0x3f) == 0 && ((1usize << (_la - 192)) & ((1usize << (NSEQ - 192)) | (1usize << (NSYMBOL - 192)) | (1usize << (NUM - 192)) | (1usize << (NUMBER - 192)) | (1usize << (NUMPROC - 192)) | (1usize << (OBJ - 192)) | (1usize << (OBJECT - 192)) | (1usize << (OF - 192)) | (1usize << (OFF - 192)) | (1usize << (OFFSET - 192)) | (1usize << (ON - 192)) | (1usize << (OP - 192)) | (1usize << (OPMARGINS - 192)) | (1usize << (OPSEQUENCE - 192)) | (1usize << (OPT - 192)) | (1usize << (OPTFILE - 192)) | (1usize << (OPTIMIZE - 192)) | (1usize << (OPTIONS - 192)) | (1usize << (OUT - 192)) | (1usize << (OUTDD - 192)) | (1usize << (PFD - 192)) | (1usize << (PPTDBG - 192)) | (1usize << (PGMN - 192)) | (1usize << (PGMNAME - 192)) | (1usize << (PROCESS - 192)) | (1usize << (PROLOG - 192)) | (1usize << (QUOTE - 192)) | (1usize << (RENT - 192)) | (1usize << (REPLACING - 192)) | (1usize << (RMODE - 192)) | (1usize << (RPARENCHAR - 192)))) != 0) || ((((_la - 224)) & !0x3f) == 0 && ((1usize << (_la - 224)) & ((1usize << (SEP - 224)) | (1usize << (SEPARATE - 224)) | (1usize << (SEQ - 224)) | (1usize << (SEQUENCE - 224)) | (1usize << (SHORT - 224)) | (1usize << (SIZE - 224)) | (1usize << (SOURCE - 224)) | (1usize << (SP - 224)) | (1usize << (SPACE - 224)) | (1usize << (SPIE - 224)) | (1usize << (SQL - 224)) | (1usize << (SQLC - 224)) | (1usize << (SQLCCSID - 224)) | (1usize << (SS - 224)) | (1usize << (SSR - 224)) | (1usize << (SSRANGE - 224)) | (1usize << (STD - 224)) | (1usize << (SYSEIB - 224)) | (1usize << (SZ - 224)) | (1usize << (TERM - 224)) | (1usize << (TERMINAL - 224)) | (1usize << (TEST - 224)) | (1usize << (THREAD - 224)) | (1usize << (TITLE - 224)) | (1usize << (TRIG - 224)) | (1usize << (TRUNC - 224)) | (1usize << (UE - 224)))) != 0) || ((((_la - 256)) & !0x3f) == 0 && ((1usize << (_la - 256)) & ((1usize << (UPPER - 256)) | (1usize << (VBREF - 256)) | (1usize << (WD - 256)) | (1usize << (XMLPARSE - 256)) | (1usize << (XMLSS - 256)) | (1usize << (XOPTS - 256)) | (1usize << (XREF - 256)) | (1usize << (YEARWINDOW - 256)) | (1usize << (YW - 256)) | (1usize << (ZWB - 256)) | (1usize << (C_CHAR - 256)) | (1usize << (D_CHAR - 256)) | (1usize << (E_CHAR - 256)) | (1usize << (F_CHAR - 256)) | (1usize << (H_CHAR - 256)) | (1usize << (I_CHAR - 256)) | (1usize << (M_CHAR - 256)) | (1usize << (N_CHAR - 256)) | (1usize << (Q_CHAR - 256)) | (1usize << (S_CHAR - 256)) | (1usize << (U_CHAR - 256)) | (1usize << (W_CHAR - 256)) | (1usize << (X_CHAR - 256)) | (1usize << (COMMACHAR - 256)) | (1usize << (DOT - 256)) | (1usize << (NONNUMERICLITERAL - 256)) | (1usize << (NUMERICLITERAL - 256)) | (1usize << (IDENTIFIER - 256)))) != 0) || ((((_la - 288)) & !0x3f) == 0 && ((1usize << (_la - 288)) & ((1usize << (FILENAME - 288)) | (1usize << (NEWLINE - 288)) | (1usize << (TEXT - 288)))) != 0) {
				{
				/*InvokeRule charData*/
				recog.base.set_state(637);
				recog.charData()?;

				}
			}

			recog.base.set_state(640);
			recog.base.match_token(DOUBLEEQUALCHAR,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- charData ----------------
pub type CharDataContextAll<'input> = CharDataContext<'input>;


pub type CharDataContext<'input> = BaseParserRuleContext<'input,CharDataContextExt<'input>>;

#[derive(Clone)]
pub struct CharDataContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CharDataContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CharDataContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_charData(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_charData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CharDataContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_charData(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_charData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_charData }
}
antlr_rust::tid!{CharDataContextExt<'a>}

impl<'input> CharDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharDataContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharDataContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CharDataContextExt<'input>>{

fn charDataLine_all(&self) ->  Vec<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn charDataLine(&self, i: usize) -> Option<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> CharDataContextAttrs<'input> for CharDataContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn charData(&mut self,)
	-> Result<Rc<CharDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_charData);
        let mut _localctx: Rc<CharDataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(644); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					recog.base.set_state(644);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
					 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
					 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
					 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
					 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
					 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG |
					 FLAGSTD | FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA |
					 JP | KA | LANG | LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT |
					 LINKAGE | LIST | LM | LONGMIXED | LONGUPPER | LPARENCHAR | LU | MAP |
					 MARGINS | MAX | MD | MDECK | MIG | MIXED | NAME | NAT | NATIONAL |
					 NATLANG | NN | NO | NOADATA | NOADV | NOALIAS | NOAWO | NOBLOCK0 |
					 NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE | NOCPSM | NOCURR |
					 NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG | NODECK |
					 NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN | NODYNAM |
					 NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL | NOF | NOFASTSRT |
					 NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT | NOGRAPHIC | NOHOOK |
					 NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP | NOMD | NOMDECK | NONAME |
					 NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF | NOOFFSET | NOOPSEQUENCE |
					 NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD | NOPROLOG | NORENT |
					 NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE | NOSQL | NOSQLC |
					 NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE | NOTERM |
					 NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD | NOX |
					 NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC | OBJ |
					 OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE | OPT |
					 OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
					 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | RPARENCHAR |
					 SEP | SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE |
					 SPIE | SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB |
					 SZ | TERM | TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE |
					 UPPER | VBREF | WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW |
					 YW | ZWB | C_CHAR | D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR |
					 N_CHAR | Q_CHAR | S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR | DOT |
					 NONNUMERICLITERAL | NUMERICLITERAL | IDENTIFIER | FILENAME | TEXT 
						=> {
							{
							/*InvokeRule charDataLine*/
							recog.base.set_state(642);
							recog.charDataLine()?;

							}
						}

					 NEWLINE 
						=> {
							{
							recog.base.set_state(643);
							recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(646); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(68,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- charDataSql ----------------
pub type CharDataSqlContextAll<'input> = CharDataSqlContext<'input>;


pub type CharDataSqlContext<'input> = BaseParserRuleContext<'input,CharDataSqlContextExt<'input>>;

#[derive(Clone)]
pub struct CharDataSqlContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CharDataSqlContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CharDataSqlContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_charDataSql(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_charDataSql(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CharDataSqlContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_charDataSql(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharDataSqlContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_charDataSql }
	//fn type_rule_index() -> usize where Self: Sized { RULE_charDataSql }
}
antlr_rust::tid!{CharDataSqlContextExt<'a>}

impl<'input> CharDataSqlContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharDataSqlContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharDataSqlContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharDataSqlContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CharDataSqlContextExt<'input>>{

fn charDataLine_all(&self) ->  Vec<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn charDataLine(&self, i: usize) -> Option<Rc<CharDataLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COPY in current rule
fn COPY_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COPY, starting from 0.
/// Returns `None` if number of children corresponding to token COPY is less or equal than `i`.
fn COPY(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COPY, i)
}
/// Retrieves all `TerminalNode`s corresponding to token REPLACE in current rule
fn REPLACE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token REPLACE, starting from 0.
/// Returns `None` if number of children corresponding to token REPLACE is less or equal than `i`.
fn REPLACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(REPLACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> CharDataSqlContextAttrs<'input> for CharDataSqlContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn charDataSql(&mut self,)
	-> Result<Rc<CharDataSqlContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharDataSqlContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_charDataSql);
        let mut _localctx: Rc<CharDataSqlContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(652); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(652);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
				 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
				 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
				 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
				 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
				 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
				 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
				 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE |
				 LIST | LM | LONGMIXED | LONGUPPER | LPARENCHAR | LU | MAP | MARGINS |
				 MAX | MD | MDECK | MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN |
				 NO | NOADATA | NOADV | NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD |
				 NOCICS | NOCMPR2 | NOCOMPILE | NOCPSM | NOCURR | NOCURRENCY | NOD |
				 NODATEPROC | NODBCS | NODE | NODEBUG | NODECK | NODIAGTRUNC | NODLL |
				 NODU | NODUMP | NODP | NODTR | NODYN | NODYNAM | NOEDF | NOEJPD | NOEPILOG |
				 NOEXIT | NOEXP | NOEXPORTALL | NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG |
				 NOFLAGSTD | NOFSRT | NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE |
				 NOLIST | NOMAP | NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ |
				 NOOBJECT | NOOFF | NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS |
				 NOP | NOPFD | NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ |
				 NOSOURCE | NOSPIE | NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE |
				 NOSTDTRUNC | NOSEQUENCE | NOTERM | NOTERMINAL | NOTEST | NOTHREAD |
				 NOTRIG | NOVBREF | NOWORD | NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL |
				 NUM | NUMBER | NUMPROC | OBJ | OBJECT | OF | OFF | OFFSET | ON | OP |
				 OPMARGINS | OPSEQUENCE | OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT |
				 OUTDD | PFD | PPTDBG | PGMN | PGMNAME | PROCESS | PROLOG | QUOTE | RENT |
				 REPLACING | RMODE | RPARENCHAR | SEP | SEPARATE | SEQ | SEQUENCE | SHORT |
				 SIZE | SOURCE | SP | SPACE | SPIE | SQL | SQLC | SQLCCSID | SS | SSR |
				 SSRANGE | STD | SYSEIB | SZ | TERM | TERMINAL | TEST | THREAD | TITLE |
				 TRIG | TRUNC | UE | UPPER | VBREF | WD | XMLPARSE | XMLSS | XOPTS |
				 XREF | YEARWINDOW | YW | ZWB | C_CHAR | D_CHAR | E_CHAR | F_CHAR | H_CHAR |
				 I_CHAR | M_CHAR | N_CHAR | Q_CHAR | S_CHAR | U_CHAR | W_CHAR | X_CHAR |
				 COMMACHAR | DOT | NONNUMERICLITERAL | NUMERICLITERAL | IDENTIFIER |
				 FILENAME | TEXT 
					=> {
						{
						/*InvokeRule charDataLine*/
						recog.base.set_state(648);
						recog.charDataLine()?;

						}
					}

				 COPY 
					=> {
						{
						recog.base.set_state(649);
						recog.base.match_token(COPY,&mut recog.err_handler)?;

						}
					}

				 REPLACE 
					=> {
						{
						recog.base.set_state(650);
						recog.base.match_token(REPLACE,&mut recog.err_handler)?;

						}
					}

				 NEWLINE 
					=> {
						{
						recog.base.set_state(651);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(654); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADATA) | (1usize << ADV) | (1usize << ALIAS) | (1usize << ANSI) | (1usize << ANY) | (1usize << APOST) | (1usize << AR) | (1usize << ARITH) | (1usize << AUTO) | (1usize << AWO) | (1usize << BIN) | (1usize << BLOCK0) | (1usize << BUF) | (1usize << BUFSIZE) | (1usize << BY) | (1usize << CBL) | (1usize << CBLCARD) | (1usize << CO) | (1usize << COBOL2) | (1usize << COBOL3) | (1usize << CODEPAGE) | (1usize << COMPAT) | (1usize << COMPILE) | (1usize << COPY) | (1usize << CP) | (1usize << CPP) | (1usize << CPSM) | (1usize << CS) | (1usize << CURR) | (1usize << CURRENCY))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (DATA - 32)) | (1usize << (DATEPROC - 32)) | (1usize << (DBCS - 32)) | (1usize << (DD - 32)) | (1usize << (DEBUG - 32)) | (1usize << (DECK - 32)) | (1usize << (DIAGTRUNC - 32)) | (1usize << (DLI - 32)) | (1usize << (DLL - 32)) | (1usize << (DP - 32)) | (1usize << (DTR - 32)) | (1usize << (DU - 32)) | (1usize << (DUMP - 32)) | (1usize << (DYN - 32)) | (1usize << (DYNAM - 32)) | (1usize << (EDF - 32)) | (1usize << (EJECT - 32)) | (1usize << (EJPD - 32)) | (1usize << (EN - 32)) | (1usize << (ENGLISH - 32)) | (1usize << (EPILOG - 32)) | (1usize << (EXCI - 32)) | (1usize << (EXIT - 32)) | (1usize << (EXP - 32)) | (1usize << (EXPORTALL - 32)) | (1usize << (EXTEND - 32)) | (1usize << (FASTSRT - 32)) | (1usize << (FLAG - 32)) | (1usize << (FLAGSTD - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (FSRT - 64)) | (1usize << (FULL - 64)) | (1usize << (GDS - 64)) | (1usize << (GRAPHIC - 64)) | (1usize << (HOOK - 64)) | (1usize << (IN - 64)) | (1usize << (INTDATE - 64)) | (1usize << (JA - 64)) | (1usize << (JP - 64)) | (1usize << (KA - 64)) | (1usize << (LANG - 64)) | (1usize << (LANGUAGE - 64)) | (1usize << (LC - 64)) | (1usize << (LENGTH - 64)) | (1usize << (LIB - 64)) | (1usize << (LILIAN - 64)) | (1usize << (LIN - 64)) | (1usize << (LINECOUNT - 64)) | (1usize << (LINKAGE - 64)) | (1usize << (LIST - 64)) | (1usize << (LM - 64)) | (1usize << (LONGMIXED - 64)) | (1usize << (LONGUPPER - 64)) | (1usize << (LPARENCHAR - 64)) | (1usize << (LU - 64)) | (1usize << (MAP - 64)) | (1usize << (MARGINS - 64)) | (1usize << (MAX - 64)) | (1usize << (MD - 64)) | (1usize << (MDECK - 64)) | (1usize << (MIG - 64)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MIXED - 96)) | (1usize << (NAME - 96)) | (1usize << (NAT - 96)) | (1usize << (NATIONAL - 96)) | (1usize << (NATLANG - 96)) | (1usize << (NN - 96)) | (1usize << (NO - 96)) | (1usize << (NOADATA - 96)) | (1usize << (NOADV - 96)) | (1usize << (NOALIAS - 96)) | (1usize << (NOAWO - 96)) | (1usize << (NOBLOCK0 - 96)) | (1usize << (NOC - 96)) | (1usize << (NOCBLCARD - 96)) | (1usize << (NOCICS - 96)) | (1usize << (NOCMPR2 - 96)) | (1usize << (NOCOMPILE - 96)) | (1usize << (NOCPSM - 96)) | (1usize << (NOCURR - 96)) | (1usize << (NOCURRENCY - 96)) | (1usize << (NOD - 96)) | (1usize << (NODATEPROC - 96)) | (1usize << (NODBCS - 96)) | (1usize << (NODE - 96)) | (1usize << (NODEBUG - 96)) | (1usize << (NODECK - 96)) | (1usize << (NODIAGTRUNC - 96)) | (1usize << (NODLL - 96)) | (1usize << (NODU - 96)) | (1usize << (NODUMP - 96)) | (1usize << (NODP - 96)) | (1usize << (NODTR - 96)))) != 0) || ((((_la - 128)) & !0x3f) == 0 && ((1usize << (_la - 128)) & ((1usize << (NODYN - 128)) | (1usize << (NODYNAM - 128)) | (1usize << (NOEDF - 128)) | (1usize << (NOEJPD - 128)) | (1usize << (NOEPILOG - 128)) | (1usize << (NOEXIT - 128)) | (1usize << (NOEXP - 128)) | (1usize << (NOEXPORTALL - 128)) | (1usize << (NOF - 128)) | (1usize << (NOFASTSRT - 128)) | (1usize << (NOFEPI - 128)) | (1usize << (NOFLAG - 128)) | (1usize << (NOFLAGMIG - 128)) | (1usize << (NOFLAGSTD - 128)) | (1usize << (NOFSRT - 128)) | (1usize << (NOGRAPHIC - 128)) | (1usize << (NOHOOK - 128)) | (1usize << (NOLENGTH - 128)) | (1usize << (NOLIB - 128)) | (1usize << (NOLINKAGE - 128)) | (1usize << (NOLIST - 128)) | (1usize << (NOMAP - 128)) | (1usize << (NOMD - 128)) | (1usize << (NOMDECK - 128)) | (1usize << (NONAME - 128)) | (1usize << (NONUM - 128)) | (1usize << (NONUMBER - 128)) | (1usize << (NOOBJ - 128)) | (1usize << (NOOBJECT - 128)) | (1usize << (NOOFF - 128)) | (1usize << (NOOFFSET - 128)) | (1usize << (NOOPSEQUENCE - 128)))) != 0) || ((((_la - 160)) & !0x3f) == 0 && ((1usize << (_la - 160)) & ((1usize << (NOOPT - 160)) | (1usize << (NOOPTIMIZE - 160)) | (1usize << (NOOPTIONS - 160)) | (1usize << (NOP - 160)) | (1usize << (NOPFD - 160)) | (1usize << (NOPROLOG - 160)) | (1usize << (NORENT - 160)) | (1usize << (NOS - 160)) | (1usize << (NOSEP - 160)) | (1usize << (NOSEPARATE - 160)) | (1usize << (NOSEQ - 160)) | (1usize << (NOSOURCE - 160)) | (1usize << (NOSPIE - 160)) | (1usize << (NOSQL - 160)) | (1usize << (NOSQLC - 160)) | (1usize << (NOSQLCCSID - 160)) | (1usize << (NOSSR - 160)) | (1usize << (NOSSRANGE - 160)) | (1usize << (NOSTDTRUNC - 160)) | (1usize << (NOSEQUENCE - 160)) | (1usize << (NOTERM - 160)) | (1usize << (NOTERMINAL - 160)) | (1usize << (NOTEST - 160)) | (1usize << (NOTHREAD - 160)) | (1usize << (NOTRIG - 160)) | (1usize << (NOVBREF - 160)) | (1usize << (NOWORD - 160)) | (1usize << (NOX - 160)) | (1usize << (NOXREF - 160)) | (1usize << (NOZWB - 160)) | (1usize << (NS - 160)))) != 0) || ((((_la - 192)) & !0x3f) == 0 && ((1usize << (_la - 192)) & ((1usize << (NSEQ - 192)) | (1usize << (NSYMBOL - 192)) | (1usize << (NUM - 192)) | (1usize << (NUMBER - 192)) | (1usize << (NUMPROC - 192)) | (1usize << (OBJ - 192)) | (1usize << (OBJECT - 192)) | (1usize << (OF - 192)) | (1usize << (OFF - 192)) | (1usize << (OFFSET - 192)) | (1usize << (ON - 192)) | (1usize << (OP - 192)) | (1usize << (OPMARGINS - 192)) | (1usize << (OPSEQUENCE - 192)) | (1usize << (OPT - 192)) | (1usize << (OPTFILE - 192)) | (1usize << (OPTIMIZE - 192)) | (1usize << (OPTIONS - 192)) | (1usize << (OUT - 192)) | (1usize << (OUTDD - 192)) | (1usize << (PFD - 192)) | (1usize << (PPTDBG - 192)) | (1usize << (PGMN - 192)) | (1usize << (PGMNAME - 192)) | (1usize << (PROCESS - 192)) | (1usize << (PROLOG - 192)) | (1usize << (QUOTE - 192)) | (1usize << (RENT - 192)) | (1usize << (REPLACE - 192)) | (1usize << (REPLACING - 192)) | (1usize << (RMODE - 192)) | (1usize << (RPARENCHAR - 192)))) != 0) || ((((_la - 224)) & !0x3f) == 0 && ((1usize << (_la - 224)) & ((1usize << (SEP - 224)) | (1usize << (SEPARATE - 224)) | (1usize << (SEQ - 224)) | (1usize << (SEQUENCE - 224)) | (1usize << (SHORT - 224)) | (1usize << (SIZE - 224)) | (1usize << (SOURCE - 224)) | (1usize << (SP - 224)) | (1usize << (SPACE - 224)) | (1usize << (SPIE - 224)) | (1usize << (SQL - 224)) | (1usize << (SQLC - 224)) | (1usize << (SQLCCSID - 224)) | (1usize << (SS - 224)) | (1usize << (SSR - 224)) | (1usize << (SSRANGE - 224)) | (1usize << (STD - 224)) | (1usize << (SYSEIB - 224)) | (1usize << (SZ - 224)) | (1usize << (TERM - 224)) | (1usize << (TERMINAL - 224)) | (1usize << (TEST - 224)) | (1usize << (THREAD - 224)) | (1usize << (TITLE - 224)) | (1usize << (TRIG - 224)) | (1usize << (TRUNC - 224)) | (1usize << (UE - 224)))) != 0) || ((((_la - 256)) & !0x3f) == 0 && ((1usize << (_la - 256)) & ((1usize << (UPPER - 256)) | (1usize << (VBREF - 256)) | (1usize << (WD - 256)) | (1usize << (XMLPARSE - 256)) | (1usize << (XMLSS - 256)) | (1usize << (XOPTS - 256)) | (1usize << (XREF - 256)) | (1usize << (YEARWINDOW - 256)) | (1usize << (YW - 256)) | (1usize << (ZWB - 256)) | (1usize << (C_CHAR - 256)) | (1usize << (D_CHAR - 256)) | (1usize << (E_CHAR - 256)) | (1usize << (F_CHAR - 256)) | (1usize << (H_CHAR - 256)) | (1usize << (I_CHAR - 256)) | (1usize << (M_CHAR - 256)) | (1usize << (N_CHAR - 256)) | (1usize << (Q_CHAR - 256)) | (1usize << (S_CHAR - 256)) | (1usize << (U_CHAR - 256)) | (1usize << (W_CHAR - 256)) | (1usize << (X_CHAR - 256)) | (1usize << (COMMACHAR - 256)) | (1usize << (DOT - 256)) | (1usize << (NONNUMERICLITERAL - 256)) | (1usize << (NUMERICLITERAL - 256)) | (1usize << (IDENTIFIER - 256)))) != 0) || ((((_la - 288)) & !0x3f) == 0 && ((1usize << (_la - 288)) & ((1usize << (FILENAME - 288)) | (1usize << (NEWLINE - 288)) | (1usize << (TEXT - 288)))) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- charDataLine ----------------
pub type CharDataLineContextAll<'input> = CharDataLineContext<'input>;


pub type CharDataLineContext<'input> = BaseParserRuleContext<'input,CharDataLineContextExt<'input>>;

#[derive(Clone)]
pub struct CharDataLineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CharDataLineContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CharDataLineContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_charDataLine(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_charDataLine(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CharDataLineContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_charDataLine(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharDataLineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_charDataLine }
	//fn type_rule_index() -> usize where Self: Sized { RULE_charDataLine }
}
antlr_rust::tid!{CharDataLineContextExt<'a>}

impl<'input> CharDataLineContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharDataLineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharDataLineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharDataLineContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CharDataLineContextExt<'input>>{

fn cobolWord_all(&self) ->  Vec<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn cobolWord(&self, i: usize) -> Option<Rc<CobolWordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn literal_all(&self) ->  Vec<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literal(&self, i: usize) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn filename_all(&self) ->  Vec<Rc<FilenameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn filename(&self, i: usize) -> Option<Rc<FilenameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token TEXT in current rule
fn TEXT_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TEXT, starting from 0.
/// Returns `None` if number of children corresponding to token TEXT is less or equal than `i`.
fn TEXT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TEXT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LPARENCHAR in current rule
fn LPARENCHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LPARENCHAR, starting from 0.
/// Returns `None` if number of children corresponding to token LPARENCHAR is less or equal than `i`.
fn LPARENCHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LPARENCHAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RPARENCHAR in current rule
fn RPARENCHAR_all(&self) -> Vec<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RPARENCHAR, starting from 0.
/// Returns `None` if number of children corresponding to token RPARENCHAR is less or equal than `i`.
fn RPARENCHAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RPARENCHAR, i)
}

}

impl<'input> CharDataLineContextAttrs<'input> for CharDataLineContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn charDataLine(&mut self,)
	-> Result<Rc<CharDataLineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharDataLineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_charDataLine);
        let mut _localctx: Rc<CharDataLineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(663); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					recog.base.set_state(663);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
					 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
					 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
					 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
					 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
					 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG |
					 FLAGSTD | FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA |
					 JP | KA | LANG | LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT |
					 LINKAGE | LIST | LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS |
					 MAX | MD | MDECK | MIG | MIXED | NAME | NAT | NATIONAL | NATLANG |
					 NN | NO | NOADATA | NOADV | NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD |
					 NOCICS | NOCMPR2 | NOCOMPILE | NOCPSM | NOCURR | NOCURRENCY | NOD |
					 NODATEPROC | NODBCS | NODE | NODEBUG | NODECK | NODIAGTRUNC | NODLL |
					 NODU | NODUMP | NODP | NODTR | NODYN | NODYNAM | NOEDF | NOEJPD | NOEPILOG |
					 NOEXIT | NOEXP | NOEXPORTALL | NOF | NOFASTSRT | NOFEPI | NOFLAG |
					 NOFLAGMIG | NOFLAGSTD | NOFSRT | NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB |
					 NOLINKAGE | NOLIST | NOMAP | NOMD | NOMDECK | NONAME | NONUM | NONUMBER |
					 NOOBJ | NOOBJECT | NOOFF | NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE |
					 NOOPTIONS | NOP | NOPFD | NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE |
					 NOSEQ | NOSOURCE | NOSPIE | NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE |
					 NOSTDTRUNC | NOSEQUENCE | NOTERM | NOTERMINAL | NOTEST | NOTHREAD |
					 NOTRIG | NOVBREF | NOWORD | NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL |
					 NUM | NUMBER | NUMPROC | OBJ | OBJECT | OF | OFF | OFFSET | ON | OP |
					 OPMARGINS | OPSEQUENCE | OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT |
					 OUTDD | PFD | PPTDBG | PGMN | PGMNAME | PROCESS | PROLOG | QUOTE |
					 RENT | REPLACING | RMODE | SEP | SEPARATE | SEQ | SEQUENCE | SHORT |
					 SIZE | SOURCE | SP | SPACE | SPIE | SQL | SQLC | SQLCCSID | SS | SSR |
					 SSRANGE | STD | SYSEIB | SZ | TERM | TERMINAL | TEST | THREAD | TITLE |
					 TRIG | TRUNC | UE | UPPER | VBREF | WD | XMLPARSE | XMLSS | XOPTS |
					 XREF | YEARWINDOW | YW | ZWB | C_CHAR | D_CHAR | E_CHAR | F_CHAR |
					 H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR | S_CHAR | U_CHAR | W_CHAR |
					 X_CHAR | COMMACHAR | IDENTIFIER 
						=> {
							{
							/*InvokeRule cobolWord*/
							recog.base.set_state(656);
							recog.cobolWord()?;

							}
						}

					 NONNUMERICLITERAL | NUMERICLITERAL 
						=> {
							{
							/*InvokeRule literal*/
							recog.base.set_state(657);
							recog.literal()?;

							}
						}

					 FILENAME 
						=> {
							{
							/*InvokeRule filename*/
							recog.base.set_state(658);
							recog.filename()?;

							}
						}

					 TEXT 
						=> {
							{
							recog.base.set_state(659);
							recog.base.match_token(TEXT,&mut recog.err_handler)?;

							}
						}

					 DOT 
						=> {
							{
							recog.base.set_state(660);
							recog.base.match_token(DOT,&mut recog.err_handler)?;

							}
						}

					 LPARENCHAR 
						=> {
							{
							recog.base.set_state(661);
							recog.base.match_token(LPARENCHAR,&mut recog.err_handler)?;

							}
						}

					 RPARENCHAR 
						=> {
							{
							recog.base.set_state(662);
							recog.base.match_token(RPARENCHAR,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(665); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(72,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cobolWord ----------------
pub type CobolWordContextAll<'input> = CobolWordContext<'input>;


pub type CobolWordContext<'input> = BaseParserRuleContext<'input,CobolWordContextExt<'input>>;

#[derive(Clone)]
pub struct CobolWordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CobolWordContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CobolWordContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cobolWord(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_cobolWord(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CobolWordContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_cobolWord(self);
	}
}

impl<'input> CustomRuleContext<'input> for CobolWordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cobolWord }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cobolWord }
}
antlr_rust::tid!{CobolWordContextExt<'a>}

impl<'input> CobolWordContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CobolWordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CobolWordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CobolWordContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CobolWordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn charDataKeyword(&self) -> Option<Rc<CharDataKeywordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CobolWordContextAttrs<'input> for CobolWordContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cobolWord(&mut self,)
	-> Result<Rc<CobolWordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CobolWordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_cobolWord);
        let mut _localctx: Rc<CobolWordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(669);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(667);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 ADATA | ADV | ALIAS | ANSI | ANY | APOST | AR | ARITH | AUTO | AWO |
			 BIN | BLOCK0 | BUF | BUFSIZE | BY | CBL | CBLCARD | CO | COBOL2 | COBOL3 |
			 CODEPAGE | COMPAT | COMPILE | CP | CPP | CPSM | CS | CURR | CURRENCY |
			 DATA | DATEPROC | DBCS | DD | DEBUG | DECK | DIAGTRUNC | DLI | DLL |
			 DP | DTR | DU | DUMP | DYN | DYNAM | EDF | EJECT | EJPD | EN | ENGLISH |
			 EPILOG | EXCI | EXIT | EXP | EXPORTALL | EXTEND | FASTSRT | FLAG | FLAGSTD |
			 FSRT | FULL | GDS | GRAPHIC | HOOK | IN | INTDATE | JA | JP | KA | LANG |
			 LANGUAGE | LC | LENGTH | LIB | LILIAN | LIN | LINECOUNT | LINKAGE | LIST |
			 LM | LONGMIXED | LONGUPPER | LU | MAP | MARGINS | MAX | MD | MDECK |
			 MIG | MIXED | NAME | NAT | NATIONAL | NATLANG | NN | NO | NOADATA | NOADV |
			 NOALIAS | NOAWO | NOBLOCK0 | NOC | NOCBLCARD | NOCICS | NOCMPR2 | NOCOMPILE |
			 NOCPSM | NOCURR | NOCURRENCY | NOD | NODATEPROC | NODBCS | NODE | NODEBUG |
			 NODECK | NODIAGTRUNC | NODLL | NODU | NODUMP | NODP | NODTR | NODYN |
			 NODYNAM | NOEDF | NOEJPD | NOEPILOG | NOEXIT | NOEXP | NOEXPORTALL |
			 NOF | NOFASTSRT | NOFEPI | NOFLAG | NOFLAGMIG | NOFLAGSTD | NOFSRT |
			 NOGRAPHIC | NOHOOK | NOLENGTH | NOLIB | NOLINKAGE | NOLIST | NOMAP |
			 NOMD | NOMDECK | NONAME | NONUM | NONUMBER | NOOBJ | NOOBJECT | NOOFF |
			 NOOFFSET | NOOPSEQUENCE | NOOPT | NOOPTIMIZE | NOOPTIONS | NOP | NOPFD |
			 NOPROLOG | NORENT | NOS | NOSEP | NOSEPARATE | NOSEQ | NOSOURCE | NOSPIE |
			 NOSQL | NOSQLC | NOSQLCCSID | NOSSR | NOSSRANGE | NOSTDTRUNC | NOSEQUENCE |
			 NOTERM | NOTERMINAL | NOTEST | NOTHREAD | NOTRIG | NOVBREF | NOWORD |
			 NOX | NOXREF | NOZWB | NS | NSEQ | NSYMBOL | NUM | NUMBER | NUMPROC |
			 OBJ | OBJECT | OF | OFF | OFFSET | ON | OP | OPMARGINS | OPSEQUENCE |
			 OPT | OPTFILE | OPTIMIZE | OPTIONS | OUT | OUTDD | PFD | PPTDBG | PGMN |
			 PGMNAME | PROCESS | PROLOG | QUOTE | RENT | REPLACING | RMODE | SEP |
			 SEPARATE | SEQ | SEQUENCE | SHORT | SIZE | SOURCE | SP | SPACE | SPIE |
			 SQL | SQLC | SQLCCSID | SS | SSR | SSRANGE | STD | SYSEIB | SZ | TERM |
			 TERMINAL | TEST | THREAD | TITLE | TRIG | TRUNC | UE | UPPER | VBREF |
			 WD | XMLPARSE | XMLSS | XOPTS | XREF | YEARWINDOW | YW | ZWB | C_CHAR |
			 D_CHAR | E_CHAR | F_CHAR | H_CHAR | I_CHAR | M_CHAR | N_CHAR | Q_CHAR |
			 S_CHAR | U_CHAR | W_CHAR | X_CHAR | COMMACHAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule charDataKeyword*/
					recog.base.set_state(668);
					recog.charDataKeyword()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NONNUMERICLITERAL
/// Returns `None` if there is no child corresponding to token NONNUMERICLITERAL
fn NONNUMERICLITERAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONNUMERICLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMERICLITERAL
/// Returns `None` if there is no child corresponding to token NUMERICLITERAL
fn NUMERICLITERAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUMERICLITERAL, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(671);
			_la = recog.base.input.la(1);
			if { !(_la==NONNUMERICLITERAL || _la==NUMERICLITERAL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- filename ----------------
pub type FilenameContextAll<'input> = FilenameContext<'input>;


pub type FilenameContext<'input> = BaseParserRuleContext<'input,FilenameContextExt<'input>>;

#[derive(Clone)]
pub struct FilenameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for FilenameContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for FilenameContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_filename(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_filename(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for FilenameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_filename(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilenameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filename }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filename }
}
antlr_rust::tid!{FilenameContextExt<'a>}

impl<'input> FilenameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FilenameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FilenameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FilenameContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<FilenameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FILENAME
/// Returns `None` if there is no child corresponding to token FILENAME
fn FILENAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FILENAME, 0)
}

}

impl<'input> FilenameContextAttrs<'input> for FilenameContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn filename(&mut self,)
	-> Result<Rc<FilenameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FilenameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_filename);
        let mut _localctx: Rc<FilenameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(673);
			recog.base.match_token(FILENAME,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- charDataKeyword ----------------
pub type CharDataKeywordContextAll<'input> = CharDataKeywordContext<'input>;


pub type CharDataKeywordContext<'input> = BaseParserRuleContext<'input,CharDataKeywordContextExt<'input>>;

#[derive(Clone)]
pub struct CharDataKeywordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Cobol85PreprocessorParserContext<'input> for CharDataKeywordContext<'input>{}

impl<'input,'a> Listenable<dyn Cobol85PreprocessorListener<'input> + 'a> for CharDataKeywordContext<'input>{
		fn enter(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_charDataKeyword(self);
		}
		fn exit(&self,listener: &mut (dyn Cobol85PreprocessorListener<'input> + 'a)) {
			listener.exit_charDataKeyword(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn Cobol85PreprocessorVisitor<'input> + 'a> for CharDataKeywordContext<'input>{
	fn accept(&self,visitor: &mut (dyn Cobol85PreprocessorVisitor<'input> + 'a)) {
		visitor.visit_charDataKeyword(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharDataKeywordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Cobol85PreprocessorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_charDataKeyword }
	//fn type_rule_index() -> usize where Self: Sized { RULE_charDataKeyword }
}
antlr_rust::tid!{CharDataKeywordContextExt<'a>}

impl<'input> CharDataKeywordContextExt<'input>{
	fn new(parent: Option<Rc<dyn Cobol85PreprocessorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharDataKeywordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharDataKeywordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharDataKeywordContextAttrs<'input>: Cobol85PreprocessorParserContext<'input> + BorrowMut<CharDataKeywordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADATA
/// Returns `None` if there is no child corresponding to token ADATA
fn ADATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ADATA, 0)
}
/// Retrieves first TerminalNode corresponding to token ADV
/// Returns `None` if there is no child corresponding to token ADV
fn ADV(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ADV, 0)
}
/// Retrieves first TerminalNode corresponding to token ALIAS
/// Returns `None` if there is no child corresponding to token ALIAS
fn ALIAS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ALIAS, 0)
}
/// Retrieves first TerminalNode corresponding to token ANSI
/// Returns `None` if there is no child corresponding to token ANSI
fn ANSI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ANSI, 0)
}
/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token APOST
/// Returns `None` if there is no child corresponding to token APOST
fn APOST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(APOST, 0)
}
/// Retrieves first TerminalNode corresponding to token AR
/// Returns `None` if there is no child corresponding to token AR
fn AR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AR, 0)
}
/// Retrieves first TerminalNode corresponding to token ARITH
/// Returns `None` if there is no child corresponding to token ARITH
fn ARITH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ARITH, 0)
}
/// Retrieves first TerminalNode corresponding to token AUTO
/// Returns `None` if there is no child corresponding to token AUTO
fn AUTO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AUTO, 0)
}
/// Retrieves first TerminalNode corresponding to token AWO
/// Returns `None` if there is no child corresponding to token AWO
fn AWO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(AWO, 0)
}
/// Retrieves first TerminalNode corresponding to token BIN
/// Returns `None` if there is no child corresponding to token BIN
fn BIN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BIN, 0)
}
/// Retrieves first TerminalNode corresponding to token BLOCK0
/// Returns `None` if there is no child corresponding to token BLOCK0
fn BLOCK0(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BLOCK0, 0)
}
/// Retrieves first TerminalNode corresponding to token BUF
/// Returns `None` if there is no child corresponding to token BUF
fn BUF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BUF, 0)
}
/// Retrieves first TerminalNode corresponding to token BUFSIZE
/// Returns `None` if there is no child corresponding to token BUFSIZE
fn BUFSIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BUFSIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(BY, 0)
}
/// Retrieves first TerminalNode corresponding to token CBL
/// Returns `None` if there is no child corresponding to token CBL
fn CBL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CBL, 0)
}
/// Retrieves first TerminalNode corresponding to token CBLCARD
/// Returns `None` if there is no child corresponding to token CBLCARD
fn CBLCARD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CBLCARD, 0)
}
/// Retrieves first TerminalNode corresponding to token CO
/// Returns `None` if there is no child corresponding to token CO
fn CO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CO, 0)
}
/// Retrieves first TerminalNode corresponding to token COBOL2
/// Returns `None` if there is no child corresponding to token COBOL2
fn COBOL2(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COBOL2, 0)
}
/// Retrieves first TerminalNode corresponding to token COBOL3
/// Returns `None` if there is no child corresponding to token COBOL3
fn COBOL3(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COBOL3, 0)
}
/// Retrieves first TerminalNode corresponding to token CODEPAGE
/// Returns `None` if there is no child corresponding to token CODEPAGE
fn CODEPAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CODEPAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMACHAR
/// Returns `None` if there is no child corresponding to token COMMACHAR
fn COMMACHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMMACHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPAT
/// Returns `None` if there is no child corresponding to token COMPAT
fn COMPAT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMPAT, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPILE
/// Returns `None` if there is no child corresponding to token COMPILE
fn COMPILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(COMPILE, 0)
}
/// Retrieves first TerminalNode corresponding to token CP
/// Returns `None` if there is no child corresponding to token CP
fn CP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CP, 0)
}
/// Retrieves first TerminalNode corresponding to token CPP
/// Returns `None` if there is no child corresponding to token CPP
fn CPP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CPP, 0)
}
/// Retrieves first TerminalNode corresponding to token CPSM
/// Returns `None` if there is no child corresponding to token CPSM
fn CPSM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CPSM, 0)
}
/// Retrieves first TerminalNode corresponding to token CS
/// Returns `None` if there is no child corresponding to token CS
fn CS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CS, 0)
}
/// Retrieves first TerminalNode corresponding to token CURR
/// Returns `None` if there is no child corresponding to token CURR
fn CURR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CURR, 0)
}
/// Retrieves first TerminalNode corresponding to token CURRENCY
/// Returns `None` if there is no child corresponding to token CURRENCY
fn CURRENCY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(CURRENCY, 0)
}
/// Retrieves first TerminalNode corresponding to token DATA
/// Returns `None` if there is no child corresponding to token DATA
fn DATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DATA, 0)
}
/// Retrieves first TerminalNode corresponding to token DATEPROC
/// Returns `None` if there is no child corresponding to token DATEPROC
fn DATEPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DATEPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token DBCS
/// Returns `None` if there is no child corresponding to token DBCS
fn DBCS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DBCS, 0)
}
/// Retrieves first TerminalNode corresponding to token DD
/// Returns `None` if there is no child corresponding to token DD
fn DD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DD, 0)
}
/// Retrieves first TerminalNode corresponding to token DEBUG
/// Returns `None` if there is no child corresponding to token DEBUG
fn DEBUG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DEBUG, 0)
}
/// Retrieves first TerminalNode corresponding to token DECK
/// Returns `None` if there is no child corresponding to token DECK
fn DECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DECK, 0)
}
/// Retrieves first TerminalNode corresponding to token DIAGTRUNC
/// Returns `None` if there is no child corresponding to token DIAGTRUNC
fn DIAGTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DIAGTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token DLI
/// Returns `None` if there is no child corresponding to token DLI
fn DLI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DLI, 0)
}
/// Retrieves first TerminalNode corresponding to token DLL
/// Returns `None` if there is no child corresponding to token DLL
fn DLL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DLL, 0)
}
/// Retrieves first TerminalNode corresponding to token DP
/// Returns `None` if there is no child corresponding to token DP
fn DP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DP, 0)
}
/// Retrieves first TerminalNode corresponding to token DTR
/// Returns `None` if there is no child corresponding to token DTR
fn DTR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DTR, 0)
}
/// Retrieves first TerminalNode corresponding to token DU
/// Returns `None` if there is no child corresponding to token DU
fn DU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DU, 0)
}
/// Retrieves first TerminalNode corresponding to token DUMP
/// Returns `None` if there is no child corresponding to token DUMP
fn DUMP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DUMP, 0)
}
/// Retrieves first TerminalNode corresponding to token DYN
/// Returns `None` if there is no child corresponding to token DYN
fn DYN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DYN, 0)
}
/// Retrieves first TerminalNode corresponding to token DYNAM
/// Returns `None` if there is no child corresponding to token DYNAM
fn DYNAM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(DYNAM, 0)
}
/// Retrieves first TerminalNode corresponding to token EDF
/// Returns `None` if there is no child corresponding to token EDF
fn EDF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EDF, 0)
}
/// Retrieves first TerminalNode corresponding to token EJECT
/// Returns `None` if there is no child corresponding to token EJECT
fn EJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token EJPD
/// Returns `None` if there is no child corresponding to token EJPD
fn EJPD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EJPD, 0)
}
/// Retrieves first TerminalNode corresponding to token EN
/// Returns `None` if there is no child corresponding to token EN
fn EN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EN, 0)
}
/// Retrieves first TerminalNode corresponding to token ENGLISH
/// Returns `None` if there is no child corresponding to token ENGLISH
fn ENGLISH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ENGLISH, 0)
}
/// Retrieves first TerminalNode corresponding to token EPILOG
/// Returns `None` if there is no child corresponding to token EPILOG
fn EPILOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EPILOG, 0)
}
/// Retrieves first TerminalNode corresponding to token EXCI
/// Returns `None` if there is no child corresponding to token EXCI
fn EXCI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXCI, 0)
}
/// Retrieves first TerminalNode corresponding to token EXIT
/// Returns `None` if there is no child corresponding to token EXIT
fn EXIT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXIT, 0)
}
/// Retrieves first TerminalNode corresponding to token EXP
/// Returns `None` if there is no child corresponding to token EXP
fn EXP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXP, 0)
}
/// Retrieves first TerminalNode corresponding to token EXPORTALL
/// Returns `None` if there is no child corresponding to token EXPORTALL
fn EXPORTALL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXPORTALL, 0)
}
/// Retrieves first TerminalNode corresponding to token EXTEND
/// Returns `None` if there is no child corresponding to token EXTEND
fn EXTEND(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(EXTEND, 0)
}
/// Retrieves first TerminalNode corresponding to token FASTSRT
/// Returns `None` if there is no child corresponding to token FASTSRT
fn FASTSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FASTSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token FLAG
/// Returns `None` if there is no child corresponding to token FLAG
fn FLAG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FLAG, 0)
}
/// Retrieves first TerminalNode corresponding to token FLAGSTD
/// Returns `None` if there is no child corresponding to token FLAGSTD
fn FLAGSTD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FLAGSTD, 0)
}
/// Retrieves first TerminalNode corresponding to token FULL
/// Returns `None` if there is no child corresponding to token FULL
fn FULL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FULL, 0)
}
/// Retrieves first TerminalNode corresponding to token FSRT
/// Returns `None` if there is no child corresponding to token FSRT
fn FSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(FSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token GDS
/// Returns `None` if there is no child corresponding to token GDS
fn GDS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(GDS, 0)
}
/// Retrieves first TerminalNode corresponding to token GRAPHIC
/// Returns `None` if there is no child corresponding to token GRAPHIC
fn GRAPHIC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(GRAPHIC, 0)
}
/// Retrieves first TerminalNode corresponding to token HOOK
/// Returns `None` if there is no child corresponding to token HOOK
fn HOOK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(HOOK, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token INTDATE
/// Returns `None` if there is no child corresponding to token INTDATE
fn INTDATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(INTDATE, 0)
}
/// Retrieves first TerminalNode corresponding to token JA
/// Returns `None` if there is no child corresponding to token JA
fn JA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(JA, 0)
}
/// Retrieves first TerminalNode corresponding to token JP
/// Returns `None` if there is no child corresponding to token JP
fn JP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(JP, 0)
}
/// Retrieves first TerminalNode corresponding to token KA
/// Returns `None` if there is no child corresponding to token KA
fn KA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(KA, 0)
}
/// Retrieves first TerminalNode corresponding to token LANG
/// Returns `None` if there is no child corresponding to token LANG
fn LANG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LANG, 0)
}
/// Retrieves first TerminalNode corresponding to token LANGUAGE
/// Returns `None` if there is no child corresponding to token LANGUAGE
fn LANGUAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LANGUAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token LC
/// Returns `None` if there is no child corresponding to token LC
fn LC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LC, 0)
}
/// Retrieves first TerminalNode corresponding to token LENGTH
/// Returns `None` if there is no child corresponding to token LENGTH
fn LENGTH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token LIB
/// Returns `None` if there is no child corresponding to token LIB
fn LIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIB, 0)
}
/// Retrieves first TerminalNode corresponding to token LILIAN
/// Returns `None` if there is no child corresponding to token LILIAN
fn LILIAN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LILIAN, 0)
}
/// Retrieves first TerminalNode corresponding to token LIN
/// Returns `None` if there is no child corresponding to token LIN
fn LIN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIN, 0)
}
/// Retrieves first TerminalNode corresponding to token LINECOUNT
/// Returns `None` if there is no child corresponding to token LINECOUNT
fn LINECOUNT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LINECOUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token LINKAGE
/// Returns `None` if there is no child corresponding to token LINKAGE
fn LINKAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LINKAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token LIST
/// Returns `None` if there is no child corresponding to token LIST
fn LIST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LIST, 0)
}
/// Retrieves first TerminalNode corresponding to token LM
/// Returns `None` if there is no child corresponding to token LM
fn LM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LM, 0)
}
/// Retrieves first TerminalNode corresponding to token LONGMIXED
/// Returns `None` if there is no child corresponding to token LONGMIXED
fn LONGMIXED(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LONGMIXED, 0)
}
/// Retrieves first TerminalNode corresponding to token LONGUPPER
/// Returns `None` if there is no child corresponding to token LONGUPPER
fn LONGUPPER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LONGUPPER, 0)
}
/// Retrieves first TerminalNode corresponding to token LU
/// Returns `None` if there is no child corresponding to token LU
fn LU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(LU, 0)
}
/// Retrieves first TerminalNode corresponding to token MAP
/// Returns `None` if there is no child corresponding to token MAP
fn MAP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MAP, 0)
}
/// Retrieves first TerminalNode corresponding to token MARGINS
/// Returns `None` if there is no child corresponding to token MARGINS
fn MARGINS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MARGINS, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX
/// Returns `None` if there is no child corresponding to token MAX
fn MAX(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token MD
/// Returns `None` if there is no child corresponding to token MD
fn MD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MD, 0)
}
/// Retrieves first TerminalNode corresponding to token MDECK
/// Returns `None` if there is no child corresponding to token MDECK
fn MDECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MDECK, 0)
}
/// Retrieves first TerminalNode corresponding to token MIG
/// Returns `None` if there is no child corresponding to token MIG
fn MIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MIG, 0)
}
/// Retrieves first TerminalNode corresponding to token MIXED
/// Returns `None` if there is no child corresponding to token MIXED
fn MIXED(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(MIXED, 0)
}
/// Retrieves first TerminalNode corresponding to token NAME
/// Returns `None` if there is no child corresponding to token NAME
fn NAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token NAT
/// Returns `None` if there is no child corresponding to token NAT
fn NAT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NAT, 0)
}
/// Retrieves first TerminalNode corresponding to token NATIONAL
/// Returns `None` if there is no child corresponding to token NATIONAL
fn NATIONAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NATIONAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NATLANG
/// Returns `None` if there is no child corresponding to token NATLANG
fn NATLANG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NATLANG, 0)
}
/// Retrieves first TerminalNode corresponding to token NN
/// Returns `None` if there is no child corresponding to token NN
fn NN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NN, 0)
}
/// Retrieves first TerminalNode corresponding to token NO
/// Returns `None` if there is no child corresponding to token NO
fn NO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NO, 0)
}
/// Retrieves first TerminalNode corresponding to token NOADATA
/// Returns `None` if there is no child corresponding to token NOADATA
fn NOADATA(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOADATA, 0)
}
/// Retrieves first TerminalNode corresponding to token NOADV
/// Returns `None` if there is no child corresponding to token NOADV
fn NOADV(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOADV, 0)
}
/// Retrieves first TerminalNode corresponding to token NOALIAS
/// Returns `None` if there is no child corresponding to token NOALIAS
fn NOALIAS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOALIAS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOAWO
/// Returns `None` if there is no child corresponding to token NOAWO
fn NOAWO(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOAWO, 0)
}
/// Retrieves first TerminalNode corresponding to token NOBLOCK0
/// Returns `None` if there is no child corresponding to token NOBLOCK0
fn NOBLOCK0(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOBLOCK0, 0)
}
/// Retrieves first TerminalNode corresponding to token NOC
/// Returns `None` if there is no child corresponding to token NOC
fn NOC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCBLCARD
/// Returns `None` if there is no child corresponding to token NOCBLCARD
fn NOCBLCARD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCBLCARD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCICS
/// Returns `None` if there is no child corresponding to token NOCICS
fn NOCICS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCICS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCMPR2
/// Returns `None` if there is no child corresponding to token NOCMPR2
fn NOCMPR2(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCMPR2, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCOMPILE
/// Returns `None` if there is no child corresponding to token NOCOMPILE
fn NOCOMPILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCOMPILE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCPSM
/// Returns `None` if there is no child corresponding to token NOCPSM
fn NOCPSM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCPSM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCURR
/// Returns `None` if there is no child corresponding to token NOCURR
fn NOCURR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCURR, 0)
}
/// Retrieves first TerminalNode corresponding to token NOCURRENCY
/// Returns `None` if there is no child corresponding to token NOCURRENCY
fn NOCURRENCY(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOCURRENCY, 0)
}
/// Retrieves first TerminalNode corresponding to token NOD
/// Returns `None` if there is no child corresponding to token NOD
fn NOD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOD, 0)
}
/// Retrieves first TerminalNode corresponding to token NODATEPROC
/// Returns `None` if there is no child corresponding to token NODATEPROC
fn NODATEPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODATEPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token NODBCS
/// Returns `None` if there is no child corresponding to token NODBCS
fn NODBCS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODBCS, 0)
}
/// Retrieves first TerminalNode corresponding to token NODE
/// Returns `None` if there is no child corresponding to token NODE
fn NODE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODE, 0)
}
/// Retrieves first TerminalNode corresponding to token NODEBUG
/// Returns `None` if there is no child corresponding to token NODEBUG
fn NODEBUG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODEBUG, 0)
}
/// Retrieves first TerminalNode corresponding to token NODECK
/// Returns `None` if there is no child corresponding to token NODECK
fn NODECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODECK, 0)
}
/// Retrieves first TerminalNode corresponding to token NODIAGTRUNC
/// Returns `None` if there is no child corresponding to token NODIAGTRUNC
fn NODIAGTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODIAGTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token NODLL
/// Returns `None` if there is no child corresponding to token NODLL
fn NODLL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODLL, 0)
}
/// Retrieves first TerminalNode corresponding to token NODU
/// Returns `None` if there is no child corresponding to token NODU
fn NODU(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODU, 0)
}
/// Retrieves first TerminalNode corresponding to token NODUMP
/// Returns `None` if there is no child corresponding to token NODUMP
fn NODUMP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODUMP, 0)
}
/// Retrieves first TerminalNode corresponding to token NODP
/// Returns `None` if there is no child corresponding to token NODP
fn NODP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODP, 0)
}
/// Retrieves first TerminalNode corresponding to token NODTR
/// Returns `None` if there is no child corresponding to token NODTR
fn NODTR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODTR, 0)
}
/// Retrieves first TerminalNode corresponding to token NODYN
/// Returns `None` if there is no child corresponding to token NODYN
fn NODYN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODYN, 0)
}
/// Retrieves first TerminalNode corresponding to token NODYNAM
/// Returns `None` if there is no child corresponding to token NODYNAM
fn NODYNAM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NODYNAM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEDF
/// Returns `None` if there is no child corresponding to token NOEDF
fn NOEDF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEDF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEJPD
/// Returns `None` if there is no child corresponding to token NOEJPD
fn NOEJPD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEJPD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEPILOG
/// Returns `None` if there is no child corresponding to token NOEPILOG
fn NOEPILOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEPILOG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXIT
/// Returns `None` if there is no child corresponding to token NOEXIT
fn NOEXIT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXIT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXP
/// Returns `None` if there is no child corresponding to token NOEXP
fn NOEXP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOEXPORTALL
/// Returns `None` if there is no child corresponding to token NOEXPORTALL
fn NOEXPORTALL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOEXPORTALL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOF
/// Returns `None` if there is no child corresponding to token NOF
fn NOF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFASTSRT
/// Returns `None` if there is no child corresponding to token NOFASTSRT
fn NOFASTSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFASTSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFEPI
/// Returns `None` if there is no child corresponding to token NOFEPI
fn NOFEPI(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFEPI, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAG
/// Returns `None` if there is no child corresponding to token NOFLAG
fn NOFLAG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAGMIG
/// Returns `None` if there is no child corresponding to token NOFLAGMIG
fn NOFLAGMIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAGMIG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFLAGSTD
/// Returns `None` if there is no child corresponding to token NOFLAGSTD
fn NOFLAGSTD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFLAGSTD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOFSRT
/// Returns `None` if there is no child corresponding to token NOFSRT
fn NOFSRT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOFSRT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOGRAPHIC
/// Returns `None` if there is no child corresponding to token NOGRAPHIC
fn NOGRAPHIC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOGRAPHIC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOHOOK
/// Returns `None` if there is no child corresponding to token NOHOOK
fn NOHOOK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOHOOK, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLENGTH
/// Returns `None` if there is no child corresponding to token NOLENGTH
fn NOLENGTH(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLIB
/// Returns `None` if there is no child corresponding to token NOLIB
fn NOLIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLIB, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLINKAGE
/// Returns `None` if there is no child corresponding to token NOLINKAGE
fn NOLINKAGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLINKAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOLIST
/// Returns `None` if there is no child corresponding to token NOLIST
fn NOLIST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOLIST, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMAP
/// Returns `None` if there is no child corresponding to token NOMAP
fn NOMAP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMAP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMD
/// Returns `None` if there is no child corresponding to token NOMD
fn NOMD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOMDECK
/// Returns `None` if there is no child corresponding to token NOMDECK
fn NOMDECK(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOMDECK, 0)
}
/// Retrieves first TerminalNode corresponding to token NONAME
/// Returns `None` if there is no child corresponding to token NONAME
fn NONAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONAME, 0)
}
/// Retrieves first TerminalNode corresponding to token NONUM
/// Returns `None` if there is no child corresponding to token NONUM
fn NONUM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONUM, 0)
}
/// Retrieves first TerminalNode corresponding to token NONUMBER
/// Returns `None` if there is no child corresponding to token NONUMBER
fn NONUMBER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NONUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOBJ
/// Returns `None` if there is no child corresponding to token NOOBJ
fn NOOBJ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOBJ, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOBJECT
/// Returns `None` if there is no child corresponding to token NOOBJECT
fn NOOBJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOFF
/// Returns `None` if there is no child corresponding to token NOOFF
fn NOOFF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOFF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOFFSET
/// Returns `None` if there is no child corresponding to token NOOFFSET
fn NOOFFSET(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPSEQUENCE
/// Returns `None` if there is no child corresponding to token NOOPSEQUENCE
fn NOOPSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPT
/// Returns `None` if there is no child corresponding to token NOOPT
fn NOOPT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPTIMIZE
/// Returns `None` if there is no child corresponding to token NOOPTIMIZE
fn NOOPTIMIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPTIMIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOOPTIONS
/// Returns `None` if there is no child corresponding to token NOOPTIONS
fn NOOPTIONS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOOPTIONS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOP
/// Returns `None` if there is no child corresponding to token NOP
fn NOP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOPFD
/// Returns `None` if there is no child corresponding to token NOPFD
fn NOPFD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOPFD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOPROLOG
/// Returns `None` if there is no child corresponding to token NOPROLOG
fn NOPROLOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOPROLOG, 0)
}
/// Retrieves first TerminalNode corresponding to token NORENT
/// Returns `None` if there is no child corresponding to token NORENT
fn NORENT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NORENT, 0)
}
/// Retrieves first TerminalNode corresponding to token NOS
/// Returns `None` if there is no child corresponding to token NOS
fn NOS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOS, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEP
/// Returns `None` if there is no child corresponding to token NOSEP
fn NOSEP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEP, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEPARATE
/// Returns `None` if there is no child corresponding to token NOSEPARATE
fn NOSEPARATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEPARATE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEQ
/// Returns `None` if there is no child corresponding to token NOSEQ
fn NOSEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSEQUENCE
/// Returns `None` if there is no child corresponding to token NOSEQUENCE
fn NOSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSOURCE
/// Returns `None` if there is no child corresponding to token NOSOURCE
fn NOSOURCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSOURCE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSPIE
/// Returns `None` if there is no child corresponding to token NOSPIE
fn NOSPIE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSPIE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQL
/// Returns `None` if there is no child corresponding to token NOSQL
fn NOSQL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQLC
/// Returns `None` if there is no child corresponding to token NOSQLC
fn NOSQLC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQLC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSQLCCSID
/// Returns `None` if there is no child corresponding to token NOSQLCCSID
fn NOSQLCCSID(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSQLCCSID, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSSR
/// Returns `None` if there is no child corresponding to token NOSSR
fn NOSSR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSSR, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSSRANGE
/// Returns `None` if there is no child corresponding to token NOSSRANGE
fn NOSSRANGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSSRANGE, 0)
}
/// Retrieves first TerminalNode corresponding to token NOSTDTRUNC
/// Returns `None` if there is no child corresponding to token NOSTDTRUNC
fn NOSTDTRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOSTDTRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTERM
/// Returns `None` if there is no child corresponding to token NOTERM
fn NOTERM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTERM, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTERMINAL
/// Returns `None` if there is no child corresponding to token NOTERMINAL
fn NOTERMINAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTERMINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTEST
/// Returns `None` if there is no child corresponding to token NOTEST
fn NOTEST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTEST, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTHREAD
/// Returns `None` if there is no child corresponding to token NOTHREAD
fn NOTHREAD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTHREAD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTRIG
/// Returns `None` if there is no child corresponding to token NOTRIG
fn NOTRIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOTRIG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOVBREF
/// Returns `None` if there is no child corresponding to token NOVBREF
fn NOVBREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOVBREF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOWORD
/// Returns `None` if there is no child corresponding to token NOWORD
fn NOWORD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOWORD, 0)
}
/// Retrieves first TerminalNode corresponding to token NOX
/// Returns `None` if there is no child corresponding to token NOX
fn NOX(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOX, 0)
}
/// Retrieves first TerminalNode corresponding to token NOXREF
/// Returns `None` if there is no child corresponding to token NOXREF
fn NOXREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOXREF, 0)
}
/// Retrieves first TerminalNode corresponding to token NOZWB
/// Returns `None` if there is no child corresponding to token NOZWB
fn NOZWB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NOZWB, 0)
}
/// Retrieves first TerminalNode corresponding to token NSEQ
/// Returns `None` if there is no child corresponding to token NSEQ
fn NSEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NSEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NSYMBOL
/// Returns `None` if there is no child corresponding to token NSYMBOL
fn NSYMBOL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NSYMBOL, 0)
}
/// Retrieves first TerminalNode corresponding to token NS
/// Returns `None` if there is no child corresponding to token NS
fn NS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NS, 0)
}
/// Retrieves first TerminalNode corresponding to token NUM
/// Returns `None` if there is no child corresponding to token NUM
fn NUM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUM, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMPROC
/// Returns `None` if there is no child corresponding to token NUMPROC
fn NUMPROC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(NUMPROC, 0)
}
/// Retrieves first TerminalNode corresponding to token OBJ
/// Returns `None` if there is no child corresponding to token OBJ
fn OBJ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OBJ, 0)
}
/// Retrieves first TerminalNode corresponding to token OBJECT
/// Returns `None` if there is no child corresponding to token OBJECT
fn OBJECT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token ON
/// Returns `None` if there is no child corresponding to token ON
fn ON(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ON, 0)
}
/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OF, 0)
}
/// Retrieves first TerminalNode corresponding to token OFF
/// Returns `None` if there is no child corresponding to token OFF
fn OFF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OFF, 0)
}
/// Retrieves first TerminalNode corresponding to token OFFSET
/// Returns `None` if there is no child corresponding to token OFFSET
fn OFFSET(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token OPMARGINS
/// Returns `None` if there is no child corresponding to token OPMARGINS
fn OPMARGINS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPMARGINS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPSEQUENCE
/// Returns `None` if there is no child corresponding to token OPSEQUENCE
fn OPSEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPSEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIMIZE
/// Returns `None` if there is no child corresponding to token OPTIMIZE
fn OPTIMIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTIMIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token OP
/// Returns `None` if there is no child corresponding to token OP
fn OP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OP, 0)
}
/// Retrieves first TerminalNode corresponding to token OPT
/// Returns `None` if there is no child corresponding to token OPT
fn OPT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTFILE
/// Returns `None` if there is no child corresponding to token OPTFILE
fn OPTFILE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTFILE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPTIONS
/// Returns `None` if there is no child corresponding to token OPTIONS
fn OPTIONS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OPTIONS, 0)
}
/// Retrieves first TerminalNode corresponding to token OUT
/// Returns `None` if there is no child corresponding to token OUT
fn OUT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OUTDD
/// Returns `None` if there is no child corresponding to token OUTDD
fn OUTDD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(OUTDD, 0)
}
/// Retrieves first TerminalNode corresponding to token PFD
/// Returns `None` if there is no child corresponding to token PFD
fn PFD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PFD, 0)
}
/// Retrieves first TerminalNode corresponding to token PGMN
/// Returns `None` if there is no child corresponding to token PGMN
fn PGMN(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PGMN, 0)
}
/// Retrieves first TerminalNode corresponding to token PGMNAME
/// Returns `None` if there is no child corresponding to token PGMNAME
fn PGMNAME(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PGMNAME, 0)
}
/// Retrieves first TerminalNode corresponding to token PPTDBG
/// Returns `None` if there is no child corresponding to token PPTDBG
fn PPTDBG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PPTDBG, 0)
}
/// Retrieves first TerminalNode corresponding to token PROCESS
/// Returns `None` if there is no child corresponding to token PROCESS
fn PROCESS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PROCESS, 0)
}
/// Retrieves first TerminalNode corresponding to token PROLOG
/// Returns `None` if there is no child corresponding to token PROLOG
fn PROLOG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(PROLOG, 0)
}
/// Retrieves first TerminalNode corresponding to token QUOTE
/// Returns `None` if there is no child corresponding to token QUOTE
fn QUOTE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token RENT
/// Returns `None` if there is no child corresponding to token RENT
fn RENT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RENT, 0)
}
/// Retrieves first TerminalNode corresponding to token REPLACING
/// Returns `None` if there is no child corresponding to token REPLACING
fn REPLACING(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(REPLACING, 0)
}
/// Retrieves first TerminalNode corresponding to token RMODE
/// Returns `None` if there is no child corresponding to token RMODE
fn RMODE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(RMODE, 0)
}
/// Retrieves first TerminalNode corresponding to token SEQ
/// Returns `None` if there is no child corresponding to token SEQ
fn SEQ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token SEQUENCE
/// Returns `None` if there is no child corresponding to token SEQUENCE
fn SEQUENCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEQUENCE, 0)
}
/// Retrieves first TerminalNode corresponding to token SEP
/// Returns `None` if there is no child corresponding to token SEP
fn SEP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEP, 0)
}
/// Retrieves first TerminalNode corresponding to token SEPARATE
/// Returns `None` if there is no child corresponding to token SEPARATE
fn SEPARATE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SEPARATE, 0)
}
/// Retrieves first TerminalNode corresponding to token SHORT
/// Returns `None` if there is no child corresponding to token SHORT
fn SHORT(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SHORT, 0)
}
/// Retrieves first TerminalNode corresponding to token SIZE
/// Returns `None` if there is no child corresponding to token SIZE
fn SIZE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token SOURCE
/// Returns `None` if there is no child corresponding to token SOURCE
fn SOURCE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SOURCE, 0)
}
/// Retrieves first TerminalNode corresponding to token SP
/// Returns `None` if there is no child corresponding to token SP
fn SP(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SP, 0)
}
/// Retrieves first TerminalNode corresponding to token SPACE
/// Returns `None` if there is no child corresponding to token SPACE
fn SPACE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SPACE, 0)
}
/// Retrieves first TerminalNode corresponding to token SPIE
/// Returns `None` if there is no child corresponding to token SPIE
fn SPIE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SPIE, 0)
}
/// Retrieves first TerminalNode corresponding to token SQL
/// Returns `None` if there is no child corresponding to token SQL
fn SQL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQL, 0)
}
/// Retrieves first TerminalNode corresponding to token SQLC
/// Returns `None` if there is no child corresponding to token SQLC
fn SQLC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQLC, 0)
}
/// Retrieves first TerminalNode corresponding to token SQLCCSID
/// Returns `None` if there is no child corresponding to token SQLCCSID
fn SQLCCSID(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SQLCCSID, 0)
}
/// Retrieves first TerminalNode corresponding to token SS
/// Returns `None` if there is no child corresponding to token SS
fn SS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SS, 0)
}
/// Retrieves first TerminalNode corresponding to token SSR
/// Returns `None` if there is no child corresponding to token SSR
fn SSR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SSR, 0)
}
/// Retrieves first TerminalNode corresponding to token SSRANGE
/// Returns `None` if there is no child corresponding to token SSRANGE
fn SSRANGE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SSRANGE, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token SYSEIB
/// Returns `None` if there is no child corresponding to token SYSEIB
fn SYSEIB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SYSEIB, 0)
}
/// Retrieves first TerminalNode corresponding to token SZ
/// Returns `None` if there is no child corresponding to token SZ
fn SZ(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(SZ, 0)
}
/// Retrieves first TerminalNode corresponding to token TERM
/// Returns `None` if there is no child corresponding to token TERM
fn TERM(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TERM, 0)
}
/// Retrieves first TerminalNode corresponding to token TERMINAL
/// Returns `None` if there is no child corresponding to token TERMINAL
fn TERMINAL(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TERMINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TEST
/// Returns `None` if there is no child corresponding to token TEST
fn TEST(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TEST, 0)
}
/// Retrieves first TerminalNode corresponding to token THREAD
/// Returns `None` if there is no child corresponding to token THREAD
fn THREAD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(THREAD, 0)
}
/// Retrieves first TerminalNode corresponding to token TITLE
/// Returns `None` if there is no child corresponding to token TITLE
fn TITLE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TITLE, 0)
}
/// Retrieves first TerminalNode corresponding to token TRIG
/// Returns `None` if there is no child corresponding to token TRIG
fn TRIG(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TRIG, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUNC
/// Returns `None` if there is no child corresponding to token TRUNC
fn TRUNC(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(TRUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token UE
/// Returns `None` if there is no child corresponding to token UE
fn UE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(UE, 0)
}
/// Retrieves first TerminalNode corresponding to token UPPER
/// Returns `None` if there is no child corresponding to token UPPER
fn UPPER(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(UPPER, 0)
}
/// Retrieves first TerminalNode corresponding to token VBREF
/// Returns `None` if there is no child corresponding to token VBREF
fn VBREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(VBREF, 0)
}
/// Retrieves first TerminalNode corresponding to token WD
/// Returns `None` if there is no child corresponding to token WD
fn WD(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(WD, 0)
}
/// Retrieves first TerminalNode corresponding to token XMLPARSE
/// Returns `None` if there is no child corresponding to token XMLPARSE
fn XMLPARSE(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XMLPARSE, 0)
}
/// Retrieves first TerminalNode corresponding to token XMLSS
/// Returns `None` if there is no child corresponding to token XMLSS
fn XMLSS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XMLSS, 0)
}
/// Retrieves first TerminalNode corresponding to token XOPTS
/// Returns `None` if there is no child corresponding to token XOPTS
fn XOPTS(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XOPTS, 0)
}
/// Retrieves first TerminalNode corresponding to token XREF
/// Returns `None` if there is no child corresponding to token XREF
fn XREF(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(XREF, 0)
}
/// Retrieves first TerminalNode corresponding to token YEARWINDOW
/// Returns `None` if there is no child corresponding to token YEARWINDOW
fn YEARWINDOW(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(YEARWINDOW, 0)
}
/// Retrieves first TerminalNode corresponding to token YW
/// Returns `None` if there is no child corresponding to token YW
fn YW(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(YW, 0)
}
/// Retrieves first TerminalNode corresponding to token ZWB
/// Returns `None` if there is no child corresponding to token ZWB
fn ZWB(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(ZWB, 0)
}
/// Retrieves first TerminalNode corresponding to token C_CHAR
/// Returns `None` if there is no child corresponding to token C_CHAR
fn C_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(C_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token D_CHAR
/// Returns `None` if there is no child corresponding to token D_CHAR
fn D_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(D_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token E_CHAR
/// Returns `None` if there is no child corresponding to token E_CHAR
fn E_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(E_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token F_CHAR
/// Returns `None` if there is no child corresponding to token F_CHAR
fn F_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(F_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token H_CHAR
/// Returns `None` if there is no child corresponding to token H_CHAR
fn H_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(H_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token I_CHAR
/// Returns `None` if there is no child corresponding to token I_CHAR
fn I_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(I_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token M_CHAR
/// Returns `None` if there is no child corresponding to token M_CHAR
fn M_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(M_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token N_CHAR
/// Returns `None` if there is no child corresponding to token N_CHAR
fn N_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(N_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token Q_CHAR
/// Returns `None` if there is no child corresponding to token Q_CHAR
fn Q_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(Q_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token S_CHAR
/// Returns `None` if there is no child corresponding to token S_CHAR
fn S_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(S_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token U_CHAR
/// Returns `None` if there is no child corresponding to token U_CHAR
fn U_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(U_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token W_CHAR
/// Returns `None` if there is no child corresponding to token W_CHAR
fn W_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(W_CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token X_CHAR
/// Returns `None` if there is no child corresponding to token X_CHAR
fn X_CHAR(&self) -> Option<Rc<TerminalNode<'input,Cobol85PreprocessorParserContextType>>> where Self:Sized{
	self.get_token(X_CHAR, 0)
}

}

impl<'input> CharDataKeywordContextAttrs<'input> for CharDataKeywordContext<'input>{}

impl<'input, I, H> Cobol85PreprocessorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn charDataKeyword(&mut self,)
	-> Result<Rc<CharDataKeywordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharDataKeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_charDataKeyword);
        let mut _localctx: Rc<CharDataKeywordContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(675);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADATA) | (1usize << ADV) | (1usize << ALIAS) | (1usize << ANSI) | (1usize << ANY) | (1usize << APOST) | (1usize << AR) | (1usize << ARITH) | (1usize << AUTO) | (1usize << AWO) | (1usize << BIN) | (1usize << BLOCK0) | (1usize << BUF) | (1usize << BUFSIZE) | (1usize << BY) | (1usize << CBL) | (1usize << CBLCARD) | (1usize << CO) | (1usize << COBOL2) | (1usize << COBOL3) | (1usize << CODEPAGE) | (1usize << COMPAT) | (1usize << COMPILE) | (1usize << CP) | (1usize << CPP) | (1usize << CPSM) | (1usize << CS) | (1usize << CURR) | (1usize << CURRENCY))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (DATA - 32)) | (1usize << (DATEPROC - 32)) | (1usize << (DBCS - 32)) | (1usize << (DD - 32)) | (1usize << (DEBUG - 32)) | (1usize << (DECK - 32)) | (1usize << (DIAGTRUNC - 32)) | (1usize << (DLI - 32)) | (1usize << (DLL - 32)) | (1usize << (DP - 32)) | (1usize << (DTR - 32)) | (1usize << (DU - 32)) | (1usize << (DUMP - 32)) | (1usize << (DYN - 32)) | (1usize << (DYNAM - 32)) | (1usize << (EDF - 32)) | (1usize << (EJECT - 32)) | (1usize << (EJPD - 32)) | (1usize << (EN - 32)) | (1usize << (ENGLISH - 32)) | (1usize << (EPILOG - 32)) | (1usize << (EXCI - 32)) | (1usize << (EXIT - 32)) | (1usize << (EXP - 32)) | (1usize << (EXPORTALL - 32)) | (1usize << (EXTEND - 32)) | (1usize << (FASTSRT - 32)) | (1usize << (FLAG - 32)) | (1usize << (FLAGSTD - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (FSRT - 64)) | (1usize << (FULL - 64)) | (1usize << (GDS - 64)) | (1usize << (GRAPHIC - 64)) | (1usize << (HOOK - 64)) | (1usize << (IN - 64)) | (1usize << (INTDATE - 64)) | (1usize << (JA - 64)) | (1usize << (JP - 64)) | (1usize << (KA - 64)) | (1usize << (LANG - 64)) | (1usize << (LANGUAGE - 64)) | (1usize << (LC - 64)) | (1usize << (LENGTH - 64)) | (1usize << (LIB - 64)) | (1usize << (LILIAN - 64)) | (1usize << (LIN - 64)) | (1usize << (LINECOUNT - 64)) | (1usize << (LINKAGE - 64)) | (1usize << (LIST - 64)) | (1usize << (LM - 64)) | (1usize << (LONGMIXED - 64)) | (1usize << (LONGUPPER - 64)) | (1usize << (LU - 64)) | (1usize << (MAP - 64)) | (1usize << (MARGINS - 64)) | (1usize << (MAX - 64)) | (1usize << (MD - 64)) | (1usize << (MDECK - 64)) | (1usize << (MIG - 64)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (MIXED - 96)) | (1usize << (NAME - 96)) | (1usize << (NAT - 96)) | (1usize << (NATIONAL - 96)) | (1usize << (NATLANG - 96)) | (1usize << (NN - 96)) | (1usize << (NO - 96)) | (1usize << (NOADATA - 96)) | (1usize << (NOADV - 96)) | (1usize << (NOALIAS - 96)) | (1usize << (NOAWO - 96)) | (1usize << (NOBLOCK0 - 96)) | (1usize << (NOC - 96)) | (1usize << (NOCBLCARD - 96)) | (1usize << (NOCICS - 96)) | (1usize << (NOCMPR2 - 96)) | (1usize << (NOCOMPILE - 96)) | (1usize << (NOCPSM - 96)) | (1usize << (NOCURR - 96)) | (1usize << (NOCURRENCY - 96)) | (1usize << (NOD - 96)) | (1usize << (NODATEPROC - 96)) | (1usize << (NODBCS - 96)) | (1usize << (NODE - 96)) | (1usize << (NODEBUG - 96)) | (1usize << (NODECK - 96)) | (1usize << (NODIAGTRUNC - 96)) | (1usize << (NODLL - 96)) | (1usize << (NODU - 96)) | (1usize << (NODUMP - 96)) | (1usize << (NODP - 96)) | (1usize << (NODTR - 96)))) != 0) || ((((_la - 128)) & !0x3f) == 0 && ((1usize << (_la - 128)) & ((1usize << (NODYN - 128)) | (1usize << (NODYNAM - 128)) | (1usize << (NOEDF - 128)) | (1usize << (NOEJPD - 128)) | (1usize << (NOEPILOG - 128)) | (1usize << (NOEXIT - 128)) | (1usize << (NOEXP - 128)) | (1usize << (NOEXPORTALL - 128)) | (1usize << (NOF - 128)) | (1usize << (NOFASTSRT - 128)) | (1usize << (NOFEPI - 128)) | (1usize << (NOFLAG - 128)) | (1usize << (NOFLAGMIG - 128)) | (1usize << (NOFLAGSTD - 128)) | (1usize << (NOFSRT - 128)) | (1usize << (NOGRAPHIC - 128)) | (1usize << (NOHOOK - 128)) | (1usize << (NOLENGTH - 128)) | (1usize << (NOLIB - 128)) | (1usize << (NOLINKAGE - 128)) | (1usize << (NOLIST - 128)) | (1usize << (NOMAP - 128)) | (1usize << (NOMD - 128)) | (1usize << (NOMDECK - 128)) | (1usize << (NONAME - 128)) | (1usize << (NONUM - 128)) | (1usize << (NONUMBER - 128)) | (1usize << (NOOBJ - 128)) | (1usize << (NOOBJECT - 128)) | (1usize << (NOOFF - 128)) | (1usize << (NOOFFSET - 128)) | (1usize << (NOOPSEQUENCE - 128)))) != 0) || ((((_la - 160)) & !0x3f) == 0 && ((1usize << (_la - 160)) & ((1usize << (NOOPT - 160)) | (1usize << (NOOPTIMIZE - 160)) | (1usize << (NOOPTIONS - 160)) | (1usize << (NOP - 160)) | (1usize << (NOPFD - 160)) | (1usize << (NOPROLOG - 160)) | (1usize << (NORENT - 160)) | (1usize << (NOS - 160)) | (1usize << (NOSEP - 160)) | (1usize << (NOSEPARATE - 160)) | (1usize << (NOSEQ - 160)) | (1usize << (NOSOURCE - 160)) | (1usize << (NOSPIE - 160)) | (1usize << (NOSQL - 160)) | (1usize << (NOSQLC - 160)) | (1usize << (NOSQLCCSID - 160)) | (1usize << (NOSSR - 160)) | (1usize << (NOSSRANGE - 160)) | (1usize << (NOSTDTRUNC - 160)) | (1usize << (NOSEQUENCE - 160)) | (1usize << (NOTERM - 160)) | (1usize << (NOTERMINAL - 160)) | (1usize << (NOTEST - 160)) | (1usize << (NOTHREAD - 160)) | (1usize << (NOTRIG - 160)) | (1usize << (NOVBREF - 160)) | (1usize << (NOWORD - 160)) | (1usize << (NOX - 160)) | (1usize << (NOXREF - 160)) | (1usize << (NOZWB - 160)) | (1usize << (NS - 160)))) != 0) || ((((_la - 192)) & !0x3f) == 0 && ((1usize << (_la - 192)) & ((1usize << (NSEQ - 192)) | (1usize << (NSYMBOL - 192)) | (1usize << (NUM - 192)) | (1usize << (NUMBER - 192)) | (1usize << (NUMPROC - 192)) | (1usize << (OBJ - 192)) | (1usize << (OBJECT - 192)) | (1usize << (OF - 192)) | (1usize << (OFF - 192)) | (1usize << (OFFSET - 192)) | (1usize << (ON - 192)) | (1usize << (OP - 192)) | (1usize << (OPMARGINS - 192)) | (1usize << (OPSEQUENCE - 192)) | (1usize << (OPT - 192)) | (1usize << (OPTFILE - 192)) | (1usize << (OPTIMIZE - 192)) | (1usize << (OPTIONS - 192)) | (1usize << (OUT - 192)) | (1usize << (OUTDD - 192)) | (1usize << (PFD - 192)) | (1usize << (PPTDBG - 192)) | (1usize << (PGMN - 192)) | (1usize << (PGMNAME - 192)) | (1usize << (PROCESS - 192)) | (1usize << (PROLOG - 192)) | (1usize << (QUOTE - 192)) | (1usize << (RENT - 192)) | (1usize << (REPLACING - 192)) | (1usize << (RMODE - 192)))) != 0) || ((((_la - 224)) & !0x3f) == 0 && ((1usize << (_la - 224)) & ((1usize << (SEP - 224)) | (1usize << (SEPARATE - 224)) | (1usize << (SEQ - 224)) | (1usize << (SEQUENCE - 224)) | (1usize << (SHORT - 224)) | (1usize << (SIZE - 224)) | (1usize << (SOURCE - 224)) | (1usize << (SP - 224)) | (1usize << (SPACE - 224)) | (1usize << (SPIE - 224)) | (1usize << (SQL - 224)) | (1usize << (SQLC - 224)) | (1usize << (SQLCCSID - 224)) | (1usize << (SS - 224)) | (1usize << (SSR - 224)) | (1usize << (SSRANGE - 224)) | (1usize << (STD - 224)) | (1usize << (SYSEIB - 224)) | (1usize << (SZ - 224)) | (1usize << (TERM - 224)) | (1usize << (TERMINAL - 224)) | (1usize << (TEST - 224)) | (1usize << (THREAD - 224)) | (1usize << (TITLE - 224)) | (1usize << (TRIG - 224)) | (1usize << (TRUNC - 224)) | (1usize << (UE - 224)))) != 0) || ((((_la - 256)) & !0x3f) == 0 && ((1usize << (_la - 256)) & ((1usize << (UPPER - 256)) | (1usize << (VBREF - 256)) | (1usize << (WD - 256)) | (1usize << (XMLPARSE - 256)) | (1usize << (XMLSS - 256)) | (1usize << (XOPTS - 256)) | (1usize << (XREF - 256)) | (1usize << (YEARWINDOW - 256)) | (1usize << (YW - 256)) | (1usize << (ZWB - 256)) | (1usize << (C_CHAR - 256)) | (1usize << (D_CHAR - 256)) | (1usize << (E_CHAR - 256)) | (1usize << (F_CHAR - 256)) | (1usize << (H_CHAR - 256)) | (1usize << (I_CHAR - 256)) | (1usize << (M_CHAR - 256)) | (1usize << (N_CHAR - 256)) | (1usize << (Q_CHAR - 256)) | (1usize << (S_CHAR - 256)) | (1usize << (U_CHAR - 256)) | (1usize << (W_CHAR - 256)) | (1usize << (X_CHAR - 256)) | (1usize << (COMMACHAR - 256)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\u{126}\u{2a8}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x03\x02\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x07\x02\x4b\x0a\x02\x0c\x02\x0e\x02\x4e\x0b\x02\x03\x02\x03\
	\x02\x03\x03\x03\x03\x05\x03\x54\x0a\x03\x03\x03\x03\x03\x06\x03\x58\x0a\
	\x03\x0d\x03\x0e\x03\x59\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\x60\x0a\
	\x04\x03\x04\x07\x04\x63\x0a\x04\x0c\x04\x0e\x04\x66\x0b\x04\x03\x04\x03\
	\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x05\x05\x7e\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x05\x05\u{97}\x0a\x05\x03\x05\x05\x05\u{9a}\x0a\x05\x03\x05\x05\
	\x05\u{9d}\x0a\x05\x03\x05\x05\x05\u{a0}\x0a\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{b4}\x0a\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{bc}\x0a\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x05\x05\u{dc}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x05\x05\u{e4}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\
	\u{ea}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{fb}\
	\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x05\x05\u{144}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{153}\
	\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x05\x05\u{169}\x0a\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{173}\x0a\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x05\x05\u{179}\x0a\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x05\x05\u{189}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x05\x05\u{192}\x0a\x05\x03\x05\x05\x05\u{195}\x0a\
	\x05\x03\x05\x05\x05\u{198}\x0a\x05\x03\x05\x05\x05\u{19b}\x0a\x05\x03\x05\
	\x05\x05\u{19e}\x0a\x05\x03\x05\x05\x05\u{1a1}\x0a\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{1b5}\x0a\
	\x05\x03\x05\x05\x05\u{1b8}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x05\x05\u{1c0}\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\
	\x06\x05\x06\u{1c7}\x0a\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\
	\x07\u{1ce}\x0a\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{1d5}\
	\x0a\x08\x03\x09\x03\x09\x03\x09\x07\x09\u{1da}\x0a\x09\x0c\x09\x0e\x09\
	\u{1dd}\x0b\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\u{1e3}\x0a\x09\x07\
	\x09\u{1e5}\x0a\x09\x0c\x09\x0e\x09\u{1e8}\x0b\x09\x03\x09\x07\x09\u{1eb}\
	\x0a\x09\x0c\x09\x0e\x09\u{1ee}\x0b\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\
	\x03\x0a\x05\x0a\u{1f5}\x0a\x0a\x03\x0a\x03\x0a\x05\x0a\u{1f9}\x0a\x0a\x03\
	\x0b\x03\x0b\x05\x0b\u{1fd}\x0a\x0b\x03\x0c\x03\x0c\x07\x0c\u{201}\x0a\x0c\
	\x0c\x0c\x0e\x0c\u{204}\x0b\x0c\x03\x0c\x03\x0c\x06\x0c\u{208}\x0a\x0c\x0d\
	\x0c\x0e\x0c\u{209}\x03\x0c\x07\x0c\u{20d}\x0a\x0c\x0c\x0c\x0e\x0c\u{210}\
	\x0b\x0c\x03\x0d\x03\x0d\x03\x0d\x07\x0d\u{215}\x0a\x0d\x0c\x0d\x0e\x0d\
	\u{218}\x0b\x0d\x03\x0d\x05\x0d\u{21b}\x0a\x0d\x03\x0e\x03\x0e\x07\x0e\u{21f}\
	\x0a\x0e\x0c\x0e\x0e\x0e\u{222}\x0b\x0e\x03\x0e\x06\x0e\u{225}\x0a\x0e\x0d\
	\x0e\x0e\x0e\u{226}\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\
	\x10\x03\x10\x07\x10\u{231}\x0a\x10\x0c\x10\x0e\x10\u{234}\x0b\x10\x03\x10\
	\x03\x10\x07\x10\u{238}\x0a\x10\x0c\x10\x0e\x10\u{23b}\x0b\x10\x03\x10\x03\
	\x10\x07\x10\u{23f}\x0a\x10\x0c\x10\x0e\x10\u{242}\x0b\x10\x03\x10\x05\x10\
	\u{245}\x0a\x10\x03\x10\x07\x10\u{248}\x0a\x10\x0c\x10\x0e\x10\u{24b}\x0b\
	\x10\x03\x10\x05\x10\u{24e}\x0a\x10\x03\x11\x03\x11\x07\x11\u{252}\x0a\x11\
	\x0c\x11\x0e\x11\u{255}\x0b\x11\x03\x11\x03\x11\x05\x11\u{259}\x0a\x11\x03\
	\x12\x03\x12\x07\x12\u{25d}\x0a\x12\x0c\x12\x0e\x12\u{260}\x0b\x12\x03\x12\
	\x03\x12\x05\x12\u{264}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\
	\u{26a}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{270}\x0a\x14\x03\
	\x15\x03\x15\x05\x15\u{274}\x0a\x15\x03\x16\x03\x16\x05\x16\u{278}\x0a\x16\
	\x03\x17\x03\x17\x03\x17\x05\x17\u{27d}\x0a\x17\x03\x18\x03\x18\x05\x18\
	\u{281}\x0a\x18\x03\x18\x03\x18\x03\x19\x03\x19\x06\x19\u{287}\x0a\x19\x0d\
	\x19\x0e\x19\u{288}\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x06\x1a\u{28f}\x0a\x1a\
	\x0d\x1a\x0e\x1a\u{290}\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\
	\x03\x1b\x06\x1b\u{29a}\x0a\x1b\x0d\x1b\x0e\x1b\u{29b}\x03\x1c\x03\x1c\x05\
	\x1c\u{2a0}\x0a\x1c\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\
	\x1f\x02\x02\x20\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\
	\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x02\x55\
	\x04\x02\x12\x12\u{da}\u{da}\x03\x02\x09\x0a\x06\x02\x19\x19\x3d\x3d\u{10e}\
	\u{10e}\u{110}\u{110}\x03\x02\x0f\x10\x04\x02\x18\x18\x1c\x1c\x04\x02\x1a\
	\x1a\u{10e}\u{10e}\x03\x02\x20\x21\x04\x02\x23\x23\x2b\x2b\x04\x02\x40\x40\
	\u{8d}\u{8d}\x04\x02\u{ba}\u{ba}\u{ff}\u{ff}\x04\x02\x27\x27\u{10f}\u{10f}\
	\x04\x02\x28\x28\x2c\x2c\x03\x02\x2d\x2e\x03\x02\x2f\x30\x03\x02\x3b\x3c\
	\x04\x02\x3e\x3e\x42\x42\x04\x02\x40\x40\u{111}\u{111}\x05\x02\u{110}\u{110}\
	\u{113}\u{113}\u{117}\u{119}\x03\x02\u{112}\u{114}\x08\x02\x25\x25\x67\x67\
	\u{f3}\u{f3}\u{10f}\u{10f}\u{115}\u{115}\u{117}\u{117}\x04\x02\x06\x06\x52\
	\x52\x03\x02\x4c\x4d\x06\x02\x1f\x1f\x34\x35\x49\x4b\u{101}\u{101}\x04\x02\
	\x4e\x4e\x54\x54\x03\x02\x5f\x60\x06\x02\x1a\x1a\x6e\x6e\x72\x72\u{10e}\
	\u{10e}\x04\x02\x05\x05\x6b\x6b\x05\x02\x1f\x1f\x34\x34\x4b\x4b\x04\x02\
	\x6e\x6e\x72\x72\x05\x02\u{110}\u{110}\u{117}\u{117}\u{119}\u{119}\x03\x02\
	\x74\x75\x04\x02\x77\x77\u{80}\u{80}\x04\x02\x76\x76\x7b\x7b\x03\x02\x7e\
	\x7f\x04\x02\x7c\x7c\u{81}\u{81}\x03\x02\u{82}\u{83}\x03\x02\u{88}\u{89}\
	\x04\x02\u{8b}\u{8b}\u{90}\u{90}\x04\x02\u{8a}\u{8a}\u{8d}\u{8d}\x03\x02\
	\u{98}\u{99}\x03\x02\u{9b}\u{9c}\x03\x02\u{9d}\u{9e}\x03\x02\u{9f}\u{a0}\
	\x03\x02\u{a2}\u{a3}\x04\x02\u{ac}\u{ac}\u{b5}\u{b5}\x04\x02\u{a9}\u{a9}\
	\u{ad}\u{ad}\x03\x02\u{b0}\u{b1}\x03\x02\u{b2}\u{b3}\x03\x02\u{b6}\u{b7}\
	\x03\x02\u{bc}\u{bd}\x04\x02\u{c1}\u{c1}\u{c3}\u{c3}\x04\x02\x24\x24\x64\
	\x65\x03\x02\u{be}\u{bf}\x03\x02\u{c4}\u{c5}\x05\x02\x61\x61\u{a6}\u{a6}\
	\u{d6}\u{d6}\x03\x02\u{c7}\u{c8}\x03\x02\u{ca}\u{cb}\x04\x02\u{d0}\u{d0}\
	\u{d2}\u{d2}\x04\x02\x43\x43\u{f6}\u{f6}\x03\x02\u{d4}\u{d5}\x03\x02\u{d8}\
	\u{d9}\x0a\x02\x15\x15\x19\x19\x57\x59\x5b\x5b\x62\x62\u{102}\u{102}\u{114}\
	\u{114}\u{118}\u{118}\x04\x02\u{dc}\u{dc}\u{116}\u{116}\x03\x02\u{e4}\u{e5}\
	\x04\x02\u{e7}\u{e7}\u{f9}\u{f9}\x04\x02\u{e8}\u{e8}\u{117}\u{117}\x03\x02\
	\u{ed}\u{ee}\x03\x02\u{f4}\u{f5}\x03\x02\u{fa}\u{fb}\x04\x02\x46\x46\u{92}\
	\u{92}\x04\x02\u{aa}\u{ab}\u{e2}\u{e3}\x04\x02\x33\x33\u{85}\u{85}\x05\x02\
	\x0d\x0d\u{d0}\u{d0}\u{f6}\u{f6}\x03\x02\u{104}\u{105}\x04\x02\u{106}\u{106}\
	\u{109}\u{109}\x06\x02\x19\x19\u{107}\u{107}\u{10e}\u{10e}\u{11a}\u{11a}\
	\x04\x02\u{10a}\u{10a}\u{11a}\u{11a}\x04\x02\x43\x43\u{e6}\u{e6}\x03\x02\
	\u{10b}\u{10c}\x04\x02\x47\x47\u{c9}\u{c9}\x03\x02\u{f0}\u{f2}\x03\x02\u{11f}\
	\u{120}\x12\x02\x03\x13\x15\x1a\x1c\x35\x37\x38\x3a\x3e\x40\x4e\x50\x59\
	\x5b\u{bb}\u{bd}\u{dd}\u{df}\u{e0}\u{e2}\u{ee}\u{f3}\u{f6}\u{f8}\u{104}\
	\u{106}\u{108}\u{10a}\u{11a}\u{11c}\u{11c}\x02\u{37d}\x02\x4c\x03\x02\x02\
	\x02\x04\x51\x03\x02\x02\x02\x06\x5b\x03\x02\x02\x02\x08\u{1bf}\x03\x02\
	\x02\x02\x0a\u{1c1}\x03\x02\x02\x02\x0c\u{1c8}\x03\x02\x02\x02\x0e\u{1cf}\
	\x03\x02\x02\x02\x10\u{1d6}\x03\x02\x02\x02\x12\u{1f4}\x03\x02\x02\x02\x14\
	\u{1fc}\x03\x02\x02\x02\x16\u{1fe}\x03\x02\x02\x02\x18\u{211}\x03\x02\x02\
	\x02\x1a\u{21c}\x03\x02\x02\x02\x1c\u{22a}\x03\x02\x02\x02\x1e\u{22e}\x03\
	\x02\x02\x02\x20\u{24f}\x03\x02\x02\x02\x22\u{25a}\x03\x02\x02\x02\x24\u{269}\
	\x03\x02\x02\x02\x26\u{26f}\x03\x02\x02\x02\x28\u{271}\x03\x02\x02\x02\x2a\
	\u{275}\x03\x02\x02\x02\x2c\u{279}\x03\x02\x02\x02\x2e\u{27e}\x03\x02\x02\
	\x02\x30\u{286}\x03\x02\x02\x02\x32\u{28e}\x03\x02\x02\x02\x34\u{299}\x03\
	\x02\x02\x02\x36\u{29f}\x03\x02\x02\x02\x38\u{2a1}\x03\x02\x02\x02\x3a\u{2a3}\
	\x03\x02\x02\x02\x3c\u{2a5}\x03\x02\x02\x02\x3e\x4b\x05\x04\x03\x02\x3f\
	\x4b\x05\x10\x09\x02\x40\x4b\x05\x0a\x06\x02\x41\x4b\x05\x0c\x07\x02\x42\
	\x4b\x05\x0e\x08\x02\x43\x4b\x05\x1c\x0f\x02\x44\x4b\x05\x18\x0d\x02\x45\
	\x4b\x05\x28\x15\x02\x46\x4b\x05\x2a\x16\x02\x47\x4b\x05\x2c\x17\x02\x48\
	\x4b\x05\x34\x1b\x02\x49\x4b\x07\u{123}\x02\x02\x4a\x3e\x03\x02\x02\x02\
	\x4a\x3f\x03\x02\x02\x02\x4a\x40\x03\x02\x02\x02\x4a\x41\x03\x02\x02\x02\
	\x4a\x42\x03\x02\x02\x02\x4a\x43\x03\x02\x02\x02\x4a\x44\x03\x02\x02\x02\
	\x4a\x45\x03\x02\x02\x02\x4a\x46\x03\x02\x02\x02\x4a\x47\x03\x02\x02\x02\
	\x4a\x48\x03\x02\x02\x02\x4a\x49\x03\x02\x02\x02\x4b\x4e\x03\x02\x02\x02\
	\x4c\x4a\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\x02\x4d\x4f\x03\x02\x02\x02\
	\x4e\x4c\x03\x02\x02\x02\x4f\x50\x07\x02\x02\x03\x50\x03\x03\x02\x02\x02\
	\x51\x57\x09\x02\x02\x02\x52\x54\x07\u{11c}\x02\x02\x53\x52\x03\x02\x02\
	\x02\x53\x54\x03\x02\x02\x02\x54\x55\x03\x02\x02\x02\x55\x58\x05\x08\x05\
	\x02\x56\x58\x05\x06\x04\x02\x57\x53\x03\x02\x02\x02\x57\x56\x03\x02\x02\
	\x02\x58\x59\x03\x02\x02\x02\x59\x57\x03\x02\x02\x02\x59\x5a\x03\x02\x02\
	\x02\x5a\x05\x03\x02\x02\x02\x5b\x5c\x07\u{108}\x02\x02\x5c\x5d\x07\x5a\
	\x02\x02\x5d\x64\x05\x08\x05\x02\x5e\x60\x07\u{11c}\x02\x02\x5f\x5e\x03\
	\x02\x02\x02\x5f\x60\x03\x02\x02\x02\x60\x61\x03\x02\x02\x02\x61\x63\x05\
	\x08\x05\x02\x62\x5f\x03\x02\x02\x02\x63\x66\x03\x02\x02\x02\x64\x62\x03\
	\x02\x02\x02\x64\x65\x03\x02\x02\x02\x65\x67\x03\x02\x02\x02\x66\x64\x03\
	\x02\x02\x02\x67\x68\x07\u{e1}\x02\x02\x68\x07\x03\x02\x02\x02\x69\u{1c0}\
	\x07\x03\x02\x02\x6a\u{1c0}\x07\x04\x02\x02\x6b\u{1c0}\x07\x08\x02\x02\x6c\
	\x6d\x09\x03\x02\x02\x6d\x6e\x07\x5a\x02\x02\x6e\x6f\x09\x04\x02\x02\x6f\
	\u{1c0}\x07\u{e1}\x02\x02\x70\u{1c0}\x07\x0c\x02\x02\x71\u{1c0}\x07\x0e\
	\x02\x02\x72\x73\x09\x05\x02\x02\x73\x74\x07\x5a\x02\x02\x74\x75\x05\x38\
	\x1d\x02\x75\x76\x07\u{e1}\x02\x02\x76\u{1c0}\x03\x02\x02\x02\x77\u{1c0}\
	\x07\x13\x02\x02\x78\x7d\x07\x14\x02\x02\x79\x7a\x07\x5a\x02\x02\x7a\x7b\
	\x05\x38\x1d\x02\x7b\x7c\x07\u{e1}\x02\x02\x7c\x7e\x03\x02\x02\x02\x7d\x79\
	\x03\x02\x02\x02\x7d\x7e\x03\x02\x02\x02\x7e\u{1c0}\x03\x02\x02\x02\x7f\
	\u{1c0}\x07\x16\x02\x02\u{80}\u{1c0}\x07\x17\x02\x02\u{81}\u{82}\x09\x06\
	\x02\x02\u{82}\u{83}\x07\x5a\x02\x02\u{83}\u{84}\x05\x38\x1d\x02\u{84}\u{85}\
	\x07\u{e1}\x02\x02\u{85}\u{1c0}\x03\x02\x02\x02\u{86}\u{1c0}\x09\x07\x02\
	\x02\u{87}\u{1c0}\x07\x1d\x02\x02\u{88}\u{1c0}\x07\x1e\x02\x02\u{89}\u{8a}\
	\x09\x08\x02\x02\u{8a}\u{8b}\x07\x5a\x02\x02\u{8b}\u{8c}\x05\x38\x1d\x02\
	\u{8c}\u{8d}\x07\u{e1}\x02\x02\u{8d}\u{1c0}\x03\x02\x02\x02\u{8e}\u{8f}\
	\x07\x22\x02\x02\u{8f}\u{90}\x07\x5a\x02\x02\u{90}\u{91}\x05\x38\x1d\x02\
	\u{91}\u{92}\x07\u{e1}\x02\x02\u{92}\u{1c0}\x03\x02\x02\x02\u{93}\u{9f}\
	\x09\x09\x02\x02\u{94}\u{96}\x07\x5a\x02\x02\u{95}\u{97}\x09\x0a\x02\x02\
	\u{96}\u{95}\x03\x02\x02\x02\u{96}\u{97}\x03\x02\x02\x02\u{97}\u{99}\x03\
	\x02\x02\x02\u{98}\u{9a}\x07\u{11c}\x02\x02\u{99}\u{98}\x03\x02\x02\x02\
	\u{99}\u{9a}\x03\x02\x02\x02\u{9a}\u{9c}\x03\x02\x02\x02\u{9b}\u{9d}\x09\
	\x0b\x02\x02\u{9c}\u{9b}\x03\x02\x02\x02\u{9c}\u{9d}\x03\x02\x02\x02\u{9d}\
	\u{9e}\x03\x02\x02\x02\u{9e}\u{a0}\x07\u{e1}\x02\x02\u{9f}\u{94}\x03\x02\
	\x02\x02\u{9f}\u{a0}\x03\x02\x02\x02\u{a0}\u{1c0}\x03\x02\x02\x02\u{a1}\
	\u{1c0}\x07\x24\x02\x02\u{a2}\u{1c0}\x09\x0c\x02\x02\u{a3}\u{1c0}\x07\x26\
	\x02\x02\u{a4}\u{1c0}\x09\x0d\x02\x02\u{a5}\u{1c0}\x07\x2a\x02\x02\u{a6}\
	\u{1c0}\x09\x0e\x02\x02\u{a7}\u{1c0}\x09\x0f\x02\x02\u{a8}\u{1c0}\x07\x31\
	\x02\x02\u{a9}\u{1c0}\x07\x37\x02\x02\u{aa}\u{1c0}\x07\x3a\x02\x02\u{ab}\
	\u{1c0}\x09\x10\x02\x02\u{ac}\u{1c0}\x09\x11\x02\x02\u{ad}\u{1c0}\x07\x3f\
	\x02\x02\u{ae}\u{af}\x09\x12\x02\x02\u{af}\u{b0}\x07\x5a\x02\x02\u{b0}\u{b3}\
	\x09\x13\x02\x02\u{b1}\u{b2}\x07\u{11c}\x02\x02\u{b2}\u{b4}\x09\x13\x02\
	\x02\u{b3}\u{b1}\x03\x02\x02\x02\u{b3}\u{b4}\x03\x02\x02\x02\u{b4}\u{b5}\
	\x03\x02\x02\x02\u{b5}\u{1c0}\x07\u{e1}\x02\x02\u{b6}\u{b7}\x07\x41\x02\
	\x02\u{b7}\u{b8}\x07\x5a\x02\x02\u{b8}\u{bb}\x09\x14\x02\x02\u{b9}\u{ba}\
	\x07\u{11c}\x02\x02\u{ba}\u{bc}\x09\x15\x02\x02\u{bb}\u{b9}\x03\x02\x02\
	\x02\u{bb}\u{bc}\x03\x02\x02\x02\u{bc}\u{bd}\x03\x02\x02\x02\u{bd}\u{1c0}\
	\x07\u{e1}\x02\x02\u{be}\u{1c0}\x07\x44\x02\x02\u{bf}\u{1c0}\x07\x45\x02\
	\x02\u{c0}\u{c1}\x07\x48\x02\x02\u{c1}\u{c2}\x07\x5a\x02\x02\u{c2}\u{c3}\
	\x09\x16\x02\x02\u{c3}\u{1c0}\x07\u{e1}\x02\x02\u{c4}\u{c5}\x09\x17\x02\
	\x02\u{c5}\u{c6}\x07\x5a\x02\x02\u{c6}\u{c7}\x09\x18\x02\x02\u{c7}\u{1c0}\
	\x07\u{e1}\x02\x02\u{c8}\u{1c0}\x07\x4f\x02\x02\u{c9}\u{1c0}\x07\x50\x02\
	\x02\u{ca}\u{1c0}\x07\x51\x02\x02\u{cb}\u{1c0}\x07\x53\x02\x02\u{cc}\u{cd}\
	\x09\x19\x02\x02\u{cd}\u{ce}\x07\x5a\x02\x02\u{ce}\u{cf}\x05\x38\x1d\x02\
	\u{cf}\u{d0}\x07\u{e1}\x02\x02\u{d0}\u{1c0}\x03\x02\x02\x02\u{d1}\u{1c0}\
	\x07\x55\x02\x02\u{d2}\u{1c0}\x07\x56\x02\x02\u{d3}\u{1c0}\x07\x5c\x02\x02\
	\u{d4}\u{d5}\x07\x5d\x02\x02\u{d5}\u{d6}\x07\x5a\x02\x02\u{d6}\u{d7}\x05\
	\x38\x1d\x02\u{d7}\u{d8}\x07\u{11c}\x02\x02\u{d8}\u{db}\x05\x38\x1d\x02\
	\u{d9}\u{da}\x07\u{11c}\x02\x02\u{da}\u{dc}\x05\x38\x1d\x02\u{db}\u{d9}\
	\x03\x02\x02\x02\u{db}\u{dc}\x03\x02\x02\x02\u{dc}\u{dd}\x03\x02\x02\x02\
	\u{dd}\u{de}\x07\u{e1}\x02\x02\u{de}\u{1c0}\x03\x02\x02\x02\u{df}\u{e3}\
	\x09\x1a\x02\x02\u{e0}\u{e1}\x07\x5a\x02\x02\u{e1}\u{e2}\x09\x1b\x02\x02\
	\u{e2}\u{e4}\x07\u{e1}\x02\x02\u{e3}\u{e0}\x03\x02\x02\x02\u{e3}\u{e4}\x03\
	\x02\x02\x02\u{e4}\u{1c0}\x03\x02\x02\x02\u{e5}\u{e9}\x07\x63\x02\x02\u{e6}\
	\u{e7}\x07\x5a\x02\x02\u{e7}\u{e8}\x09\x1c\x02\x02\u{e8}\u{ea}\x07\u{e1}\
	\x02\x02\u{e9}\u{e6}\x03\x02\x02\x02\u{e9}\u{ea}\x03\x02\x02\x02\u{ea}\u{1c0}\
	\x03\x02\x02\x02\u{eb}\u{ec}\x07\x66\x02\x02\u{ec}\u{ed}\x07\x5a\x02\x02\
	\u{ed}\u{ee}\x09\x1d\x02\x02\u{ee}\u{1c0}\x07\u{e1}\x02\x02\u{ef}\u{1c0}\
	\x07\x69\x02\x02\u{f0}\u{1c0}\x07\x6a\x02\x02\u{f1}\u{1c0}\x07\x6c\x02\x02\
	\u{f2}\u{1c0}\x07\x6d\x02\x02\u{f3}\u{1c0}\x07\x6f\x02\x02\u{f4}\u{1c0}\
	\x07\x70\x02\x02\u{f5}\u{1c0}\x07\x71\x02\x02\u{f6}\u{fa}\x09\x1e\x02\x02\
	\u{f7}\u{f8}\x07\x5a\x02\x02\u{f8}\u{f9}\x09\x1f\x02\x02\u{f9}\u{fb}\x07\
	\u{e1}\x02\x02\u{fa}\u{f7}\x03\x02\x02\x02\u{fa}\u{fb}\x03\x02\x02\x02\u{fb}\
	\u{1c0}\x03\x02\x02\x02\u{fc}\u{1c0}\x07\x73\x02\x02\u{fd}\u{1c0}\x09\x20\
	\x02\x02\u{fe}\u{1c0}\x09\x21\x02\x02\u{ff}\u{1c0}\x07\x78\x02\x02\u{100}\
	\u{1c0}\x07\x7a\x02\x02\u{101}\u{1c0}\x09\x22\x02\x02\u{102}\u{1c0}\x07\
	\x7d\x02\x02\u{103}\u{1c0}\x07\x79\x02\x02\u{104}\u{1c0}\x09\x23\x02\x02\
	\u{105}\u{1c0}\x09\x24\x02\x02\u{106}\u{1c0}\x09\x25\x02\x02\u{107}\u{1c0}\
	\x07\u{84}\x02\x02\u{108}\u{1c0}\x07\u{86}\x02\x02\u{109}\u{1c0}\x07\u{87}\
	\x02\x02\u{10a}\u{1c0}\x09\x26\x02\x02\u{10b}\u{1c0}\x09\x27\x02\x02\u{10c}\
	\u{1c0}\x07\u{8c}\x02\x02\u{10d}\u{1c0}\x09\x28\x02\x02\u{10e}\u{1c0}\x07\
	\u{8e}\x02\x02\u{10f}\u{1c0}\x07\u{8f}\x02\x02\u{110}\u{1c0}\x07\u{91}\x02\
	\x02\u{111}\u{1c0}\x07\u{93}\x02\x02\u{112}\u{1c0}\x07\u{94}\x02\x02\u{113}\
	\u{1c0}\x07\u{95}\x02\x02\u{114}\u{1c0}\x07\u{96}\x02\x02\u{115}\u{1c0}\
	\x07\u{97}\x02\x02\u{116}\u{1c0}\x09\x29\x02\x02\u{117}\u{1c0}\x07\u{9a}\
	\x02\x02\u{118}\u{1c0}\x09\x2a\x02\x02\u{119}\u{1c0}\x09\x2b\x02\x02\u{11a}\
	\u{1c0}\x09\x2c\x02\x02\u{11b}\u{1c0}\x07\u{a1}\x02\x02\u{11c}\u{1c0}\x09\
	\x2d\x02\x02\u{11d}\u{1c0}\x07\u{a4}\x02\x02\u{11e}\u{1c0}\x07\u{a5}\x02\
	\x02\u{11f}\u{1c0}\x07\u{a7}\x02\x02\u{120}\u{1c0}\x07\u{a8}\x02\x02\u{121}\
	\u{1c0}\x09\x2e\x02\x02\u{122}\u{1c0}\x09\x2f\x02\x02\u{123}\u{1c0}\x07\
	\u{ae}\x02\x02\u{124}\u{1c0}\x07\u{af}\x02\x02\u{125}\u{1c0}\x09\x30\x02\
	\x02\u{126}\u{1c0}\x09\x31\x02\x02\u{127}\u{1c0}\x07\u{b4}\x02\x02\u{128}\
	\u{1c0}\x09\x32\x02\x02\u{129}\u{1c0}\x07\u{b8}\x02\x02\u{12a}\u{1c0}\x07\
	\u{b9}\x02\x02\u{12b}\u{1c0}\x07\u{bb}\x02\x02\u{12c}\u{1c0}\x09\x33\x02\
	\x02\u{12d}\u{1c0}\x07\u{c2}\x02\x02\u{12e}\u{12f}\x09\x34\x02\x02\u{12f}\
	\u{130}\x07\x5a\x02\x02\u{130}\u{131}\x09\x35\x02\x02\u{131}\u{1c0}\x07\
	\u{e1}\x02\x02\u{132}\u{1c0}\x07\u{bb}\x02\x02\u{133}\u{1c0}\x09\x36\x02\
	\x02\u{134}\u{1c0}\x07\u{c0}\x02\x02\u{135}\u{1c0}\x09\x37\x02\x02\u{136}\
	\u{137}\x07\u{c6}\x02\x02\u{137}\u{138}\x07\x5a\x02\x02\u{138}\u{139}\x09\
	\x38\x02\x02\u{139}\u{1c0}\x07\u{e1}\x02\x02\u{13a}\u{1c0}\x09\x39\x02\x02\
	\u{13b}\u{1c0}\x09\x3a\x02\x02\u{13c}\u{13d}\x07\u{ce}\x02\x02\u{13d}\u{13e}\
	\x07\x5a\x02\x02\u{13e}\u{13f}\x05\x38\x1d\x02\u{13f}\u{140}\x07\u{11c}\
	\x02\x02\u{140}\u{143}\x05\x38\x1d\x02\u{141}\u{142}\x07\u{11c}\x02\x02\
	\u{142}\u{144}\x05\x38\x1d\x02\u{143}\u{141}\x03\x02\x02\x02\u{143}\u{144}\
	\x03\x02\x02\x02\u{144}\u{145}\x03\x02\x02\x02\u{145}\u{146}\x07\u{e1}\x02\
	\x02\u{146}\u{1c0}\x03\x02\x02\x02\u{147}\u{148}\x07\u{cf}\x02\x02\u{148}\
	\u{149}\x07\x5a\x02\x02\u{149}\u{14a}\x05\x38\x1d\x02\u{14a}\u{14b}\x07\
	\u{11c}\x02\x02\u{14b}\u{14c}\x05\x38\x1d\x02\u{14c}\u{14d}\x07\u{e1}\x02\
	\x02\u{14d}\u{1c0}\x03\x02\x02\x02\u{14e}\u{152}\x09\x3b\x02\x02\u{14f}\
	\u{150}\x07\x5a\x02\x02\u{150}\u{151}\x09\x3c\x02\x02\u{151}\u{153}\x07\
	\u{e1}\x02\x02\u{152}\u{14f}\x03\x02\x02\x02\u{152}\u{153}\x03\x02\x02\x02\
	\u{153}\u{1c0}\x03\x02\x02\x02\u{154}\u{1c0}\x07\u{d1}\x02\x02\u{155}\u{1c0}\
	\x07\u{d3}\x02\x02\u{156}\u{1c0}\x07\u{cd}\x02\x02\u{157}\u{158}\x09\x3d\
	\x02\x02\u{158}\u{159}\x07\x5a\x02\x02\u{159}\u{15a}\x05\x36\x1c\x02\u{15a}\
	\u{15b}\x07\u{e1}\x02\x02\u{15b}\u{1c0}\x03\x02\x02\x02\u{15c}\u{15d}\x09\
	\x3e\x02\x02\u{15d}\u{15e}\x07\x5a\x02\x02\u{15e}\u{15f}\x09\x3f\x02\x02\
	\u{15f}\u{1c0}\x07\u{e1}\x02\x02\u{160}\u{1c0}\x07\u{db}\x02\x02\u{161}\
	\u{1c0}\x09\x40\x02\x02\u{162}\u{1c0}\x07\u{dd}\x02\x02\u{163}\u{164}\x07\
	\u{e0}\x02\x02\u{164}\u{168}\x07\x5a\x02\x02\u{165}\u{169}\x07\x07\x02\x02\
	\u{166}\u{169}\x07\x0b\x02\x02\u{167}\u{169}\x05\x38\x1d\x02\u{168}\u{165}\
	\x03\x02\x02\x02\u{168}\u{166}\x03\x02\x02\x02\u{168}\u{167}\x03\x02\x02\
	\x02\u{169}\u{16a}\x03\x02\x02\x02\u{16a}\u{1c0}\x07\u{e1}\x02\x02\u{16b}\
	\u{172}\x09\x41\x02\x02\u{16c}\u{16d}\x07\x5a\x02\x02\u{16d}\u{16e}\x05\
	\x38\x1d\x02\u{16e}\u{16f}\x07\u{11c}\x02\x02\u{16f}\u{170}\x05\x38\x1d\
	\x02\u{170}\u{171}\x07\u{e1}\x02\x02\u{171}\u{173}\x03\x02\x02\x02\u{172}\
	\u{16c}\x03\x02\x02\x02\u{172}\u{173}\x03\x02\x02\x02\u{173}\u{1c0}\x03\
	\x02\x02\x02\u{174}\u{175}\x09\x42\x02\x02\u{175}\u{178}\x07\x5a\x02\x02\
	\u{176}\u{179}\x07\x5e\x02\x02\u{177}\u{179}\x05\x38\x1d\x02\u{178}\u{176}\
	\x03\x02\x02\x02\u{178}\u{177}\x03\x02\x02\x02\u{179}\u{17a}\x03\x02\x02\
	\x02\u{17a}\u{1c0}\x07\u{e1}\x02\x02\u{17b}\u{1c0}\x09\x43\x02\x02\u{17c}\
	\u{1c0}\x07\u{e9}\x02\x02\u{17d}\u{17e}\x07\u{ea}\x02\x02\u{17e}\u{17f}\
	\x07\x5a\x02\x02\u{17f}\u{180}\x05\x38\x1d\x02\u{180}\u{181}\x07\u{e1}\x02\
	\x02\u{181}\u{1c0}\x03\x02\x02\x02\u{182}\u{1c0}\x07\u{eb}\x02\x02\u{183}\
	\u{188}\x07\u{ec}\x02\x02\u{184}\u{185}\x07\x5a\x02\x02\u{185}\u{186}\x05\
	\x38\x1d\x02\u{186}\u{187}\x07\u{e1}\x02\x02\u{187}\u{189}\x03\x02\x02\x02\
	\u{188}\u{184}\x03\x02\x02\x02\u{188}\u{189}\x03\x02\x02\x02\u{189}\u{1c0}\
	\x03\x02\x02\x02\u{18a}\u{1c0}\x09\x44\x02\x02\u{18b}\u{1c0}\x09\x45\x02\
	\x02\u{18c}\u{1c0}\x07\u{f8}\x02\x02\u{18d}\u{1c0}\x09\x46\x02\x02\u{18e}\
	\u{1a0}\x07\u{fc}\x02\x02\u{18f}\u{191}\x07\x5a\x02\x02\u{190}\u{192}\x09\
	\x47\x02\x02\u{191}\u{190}\x03\x02\x02\x02\u{191}\u{192}\x03\x02\x02\x02\
	\u{192}\u{194}\x03\x02\x02\x02\u{193}\u{195}\x07\u{11c}\x02\x02\u{194}\u{193}\
	\x03\x02\x02\x02\u{194}\u{195}\x03\x02\x02\x02\u{195}\u{197}\x03\x02\x02\
	\x02\u{196}\u{198}\x09\x48\x02\x02\u{197}\u{196}\x03\x02\x02\x02\u{197}\
	\u{198}\x03\x02\x02\x02\u{198}\u{19a}\x03\x02\x02\x02\u{199}\u{19b}\x07\
	\u{11c}\x02\x02\u{19a}\u{199}\x03\x02\x02\x02\u{19a}\u{19b}\x03\x02\x02\
	\x02\u{19b}\u{19d}\x03\x02\x02\x02\u{19c}\u{19e}\x09\x49\x02\x02\u{19d}\
	\u{19c}\x03\x02\x02\x02\u{19d}\u{19e}\x03\x02\x02\x02\u{19e}\u{19f}\x03\
	\x02\x02\x02\u{19f}\u{1a1}\x07\u{e1}\x02\x02\u{1a0}\u{18f}\x03\x02\x02\x02\
	\u{1a0}\u{1a1}\x03\x02\x02\x02\u{1a1}\u{1c0}\x03\x02\x02\x02\u{1a2}\u{1c0}\
	\x07\u{fd}\x02\x02\u{1a3}\u{1a4}\x07\u{100}\x02\x02\u{1a4}\u{1a5}\x07\x5a\
	\x02\x02\u{1a5}\u{1a6}\x09\x4a\x02\x02\u{1a6}\u{1c0}\x07\u{e1}\x02\x02\u{1a7}\
	\u{1c0}\x07\u{103}\x02\x02\u{1a8}\u{1a9}\x09\x4b\x02\x02\u{1a9}\u{1aa}\x07\
	\x5a\x02\x02\u{1aa}\u{1ab}\x05\x36\x1c\x02\u{1ab}\u{1ac}\x07\u{e1}\x02\x02\
	\u{1ac}\u{1c0}\x03\x02\x02\x02\u{1ad}\u{1ae}\x09\x4c\x02\x02\u{1ae}\u{1af}\
	\x07\x5a\x02\x02\u{1af}\u{1b0}\x09\x4d\x02\x02\u{1b0}\u{1c0}\x07\u{e1}\x02\
	\x02\u{1b1}\u{1b7}\x09\x4e\x02\x02\u{1b2}\u{1b4}\x07\x5a\x02\x02\u{1b3}\
	\u{1b5}\x09\x4f\x02\x02\u{1b4}\u{1b3}\x03\x02\x02\x02\u{1b4}\u{1b5}\x03\
	\x02\x02\x02\u{1b5}\u{1b6}\x03\x02\x02\x02\u{1b6}\u{1b8}\x07\u{e1}\x02\x02\
	\u{1b7}\u{1b2}\x03\x02\x02\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1c0}\
	\x03\x02\x02\x02\u{1b9}\u{1ba}\x09\x50\x02\x02\u{1ba}\u{1bb}\x07\x5a\x02\
	\x02\u{1bb}\u{1bc}\x05\x38\x1d\x02\u{1bc}\u{1bd}\x07\u{e1}\x02\x02\u{1bd}\
	\u{1c0}\x03\x02\x02\x02\u{1be}\u{1c0}\x07\u{10d}\x02\x02\u{1bf}\x69\x03\
	\x02\x02\x02\u{1bf}\x6a\x03\x02\x02\x02\u{1bf}\x6b\x03\x02\x02\x02\u{1bf}\
	\x6c\x03\x02\x02\x02\u{1bf}\x70\x03\x02\x02\x02\u{1bf}\x71\x03\x02\x02\x02\
	\u{1bf}\x72\x03\x02\x02\x02\u{1bf}\x77\x03\x02\x02\x02\u{1bf}\x78\x03\x02\
	\x02\x02\u{1bf}\x7f\x03\x02\x02\x02\u{1bf}\u{80}\x03\x02\x02\x02\u{1bf}\
	\u{81}\x03\x02\x02\x02\u{1bf}\u{86}\x03\x02\x02\x02\u{1bf}\u{87}\x03\x02\
	\x02\x02\u{1bf}\u{88}\x03\x02\x02\x02\u{1bf}\u{89}\x03\x02\x02\x02\u{1bf}\
	\u{8e}\x03\x02\x02\x02\u{1bf}\u{93}\x03\x02\x02\x02\u{1bf}\u{a1}\x03\x02\
	\x02\x02\u{1bf}\u{a2}\x03\x02\x02\x02\u{1bf}\u{a3}\x03\x02\x02\x02\u{1bf}\
	\u{a4}\x03\x02\x02\x02\u{1bf}\u{a5}\x03\x02\x02\x02\u{1bf}\u{a6}\x03\x02\
	\x02\x02\u{1bf}\u{a7}\x03\x02\x02\x02\u{1bf}\u{a8}\x03\x02\x02\x02\u{1bf}\
	\u{a9}\x03\x02\x02\x02\u{1bf}\u{aa}\x03\x02\x02\x02\u{1bf}\u{ab}\x03\x02\
	\x02\x02\u{1bf}\u{ac}\x03\x02\x02\x02\u{1bf}\u{ad}\x03\x02\x02\x02\u{1bf}\
	\u{ae}\x03\x02\x02\x02\u{1bf}\u{b6}\x03\x02\x02\x02\u{1bf}\u{be}\x03\x02\
	\x02\x02\u{1bf}\u{bf}\x03\x02\x02\x02\u{1bf}\u{c0}\x03\x02\x02\x02\u{1bf}\
	\u{c4}\x03\x02\x02\x02\u{1bf}\u{c8}\x03\x02\x02\x02\u{1bf}\u{c9}\x03\x02\
	\x02\x02\u{1bf}\u{ca}\x03\x02\x02\x02\u{1bf}\u{cb}\x03\x02\x02\x02\u{1bf}\
	\u{cc}\x03\x02\x02\x02\u{1bf}\u{d1}\x03\x02\x02\x02\u{1bf}\u{d2}\x03\x02\
	\x02\x02\u{1bf}\u{d3}\x03\x02\x02\x02\u{1bf}\u{d4}\x03\x02\x02\x02\u{1bf}\
	\u{df}\x03\x02\x02\x02\u{1bf}\u{e5}\x03\x02\x02\x02\u{1bf}\u{eb}\x03\x02\
	\x02\x02\u{1bf}\u{ef}\x03\x02\x02\x02\u{1bf}\u{f0}\x03\x02\x02\x02\u{1bf}\
	\u{f1}\x03\x02\x02\x02\u{1bf}\u{f2}\x03\x02\x02\x02\u{1bf}\u{f3}\x03\x02\
	\x02\x02\u{1bf}\u{f4}\x03\x02\x02\x02\u{1bf}\u{f5}\x03\x02\x02\x02\u{1bf}\
	\u{f6}\x03\x02\x02\x02\u{1bf}\u{fc}\x03\x02\x02\x02\u{1bf}\u{fd}\x03\x02\
	\x02\x02\u{1bf}\u{fe}\x03\x02\x02\x02\u{1bf}\u{ff}\x03\x02\x02\x02\u{1bf}\
	\u{100}\x03\x02\x02\x02\u{1bf}\u{101}\x03\x02\x02\x02\u{1bf}\u{102}\x03\
	\x02\x02\x02\u{1bf}\u{103}\x03\x02\x02\x02\u{1bf}\u{104}\x03\x02\x02\x02\
	\u{1bf}\u{105}\x03\x02\x02\x02\u{1bf}\u{106}\x03\x02\x02\x02\u{1bf}\u{107}\
	\x03\x02\x02\x02\u{1bf}\u{108}\x03\x02\x02\x02\u{1bf}\u{109}\x03\x02\x02\
	\x02\u{1bf}\u{10a}\x03\x02\x02\x02\u{1bf}\u{10b}\x03\x02\x02\x02\u{1bf}\
	\u{10c}\x03\x02\x02\x02\u{1bf}\u{10d}\x03\x02\x02\x02\u{1bf}\u{10e}\x03\
	\x02\x02\x02\u{1bf}\u{10f}\x03\x02\x02\x02\u{1bf}\u{110}\x03\x02\x02\x02\
	\u{1bf}\u{111}\x03\x02\x02\x02\u{1bf}\u{112}\x03\x02\x02\x02\u{1bf}\u{113}\
	\x03\x02\x02\x02\u{1bf}\u{114}\x03\x02\x02\x02\u{1bf}\u{115}\x03\x02\x02\
	\x02\u{1bf}\u{116}\x03\x02\x02\x02\u{1bf}\u{117}\x03\x02\x02\x02\u{1bf}\
	\u{118}\x03\x02\x02\x02\u{1bf}\u{119}\x03\x02\x02\x02\u{1bf}\u{11a}\x03\
	\x02\x02\x02\u{1bf}\u{11b}\x03\x02\x02\x02\u{1bf}\u{11c}\x03\x02\x02\x02\
	\u{1bf}\u{11d}\x03\x02\x02\x02\u{1bf}\u{11e}\x03\x02\x02\x02\u{1bf}\u{11f}\
	\x03\x02\x02\x02\u{1bf}\u{120}\x03\x02\x02\x02\u{1bf}\u{121}\x03\x02\x02\
	\x02\u{1bf}\u{122}\x03\x02\x02\x02\u{1bf}\u{123}\x03\x02\x02\x02\u{1bf}\
	\u{124}\x03\x02\x02\x02\u{1bf}\u{125}\x03\x02\x02\x02\u{1bf}\u{126}\x03\
	\x02\x02\x02\u{1bf}\u{127}\x03\x02\x02\x02\u{1bf}\u{128}\x03\x02\x02\x02\
	\u{1bf}\u{129}\x03\x02\x02\x02\u{1bf}\u{12a}\x03\x02\x02\x02\u{1bf}\u{12b}\
	\x03\x02\x02\x02\u{1bf}\u{12c}\x03\x02\x02\x02\u{1bf}\u{12d}\x03\x02\x02\
	\x02\u{1bf}\u{12e}\x03\x02\x02\x02\u{1bf}\u{132}\x03\x02\x02\x02\u{1bf}\
	\u{133}\x03\x02\x02\x02\u{1bf}\u{134}\x03\x02\x02\x02\u{1bf}\u{135}\x03\
	\x02\x02\x02\u{1bf}\u{136}\x03\x02\x02\x02\u{1bf}\u{13a}\x03\x02\x02\x02\
	\u{1bf}\u{13b}\x03\x02\x02\x02\u{1bf}\u{13c}\x03\x02\x02\x02\u{1bf}\u{147}\
	\x03\x02\x02\x02\u{1bf}\u{14e}\x03\x02\x02\x02\u{1bf}\u{154}\x03\x02\x02\
	\x02\u{1bf}\u{155}\x03\x02\x02\x02\u{1bf}\u{156}\x03\x02\x02\x02\u{1bf}\
	\u{157}\x03\x02\x02\x02\u{1bf}\u{15c}\x03\x02\x02\x02\u{1bf}\u{160}\x03\
	\x02\x02\x02\u{1bf}\u{161}\x03\x02\x02\x02\u{1bf}\u{162}\x03\x02\x02\x02\
	\u{1bf}\u{163}\x03\x02\x02\x02\u{1bf}\u{16b}\x03\x02\x02\x02\u{1bf}\u{174}\
	\x03\x02\x02\x02\u{1bf}\u{17b}\x03\x02\x02\x02\u{1bf}\u{17c}\x03\x02\x02\
	\x02\u{1bf}\u{17d}\x03\x02\x02\x02\u{1bf}\u{182}\x03\x02\x02\x02\u{1bf}\
	\u{183}\x03\x02\x02\x02\u{1bf}\u{18a}\x03\x02\x02\x02\u{1bf}\u{18b}\x03\
	\x02\x02\x02\u{1bf}\u{18c}\x03\x02\x02\x02\u{1bf}\u{18d}\x03\x02\x02\x02\
	\u{1bf}\u{18e}\x03\x02\x02\x02\u{1bf}\u{1a2}\x03\x02\x02\x02\u{1bf}\u{1a3}\
	\x03\x02\x02\x02\u{1bf}\u{1a7}\x03\x02\x02\x02\u{1bf}\u{1a8}\x03\x02\x02\
	\x02\u{1bf}\u{1ad}\x03\x02\x02\x02\u{1bf}\u{1b1}\x03\x02\x02\x02\u{1bf}\
	\u{1b9}\x03\x02\x02\x02\u{1bf}\u{1be}\x03\x02\x02\x02\u{1c0}\x09\x03\x02\
	\x02\x02\u{1c1}\u{1c2}\x07\x39\x02\x02\u{1c2}\u{1c3}\x07\x14\x02\x02\u{1c3}\
	\u{1c4}\x05\x30\x19\x02\u{1c4}\u{1c6}\x07\x36\x02\x02\u{1c5}\u{1c7}\x07\
	\u{11d}\x02\x02\u{1c6}\u{1c5}\x03\x02\x02\x02\u{1c6}\u{1c7}\x03\x02\x02\
	\x02\u{1c7}\x0b\x03\x02\x02\x02\u{1c8}\u{1c9}\x07\x39\x02\x02\u{1c9}\u{1ca}\
	\x07\u{ec}\x02\x02\u{1ca}\u{1cb}\x05\x32\x1a\x02\u{1cb}\u{1cd}\x07\x36\x02\
	\x02\u{1cc}\u{1ce}\x07\u{11d}\x02\x02\u{1cd}\u{1cc}\x03\x02\x02\x02\u{1cd}\
	\u{1ce}\x03\x02\x02\x02\u{1ce}\x0d\x03\x02\x02\x02\u{1cf}\u{1d0}\x07\x39\
	\x02\x02\u{1d0}\u{1d1}\x07\u{ef}\x02\x02\u{1d1}\u{1d2}\x05\x30\x19\x02\u{1d2}\
	\u{1d4}\x07\x36\x02\x02\u{1d3}\u{1d5}\x07\u{11d}\x02\x02\u{1d4}\u{1d3}\x03\
	\x02\x02\x02\u{1d4}\u{1d5}\x03\x02\x02\x02\u{1d5}\x0f\x03\x02\x02\x02\u{1d6}\
	\u{1d7}\x07\x1b\x02\x02\u{1d7}\u{1e6}\x05\x12\x0a\x02\u{1d8}\u{1da}\x07\
	\u{123}\x02\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\u{1da}\u{1dd}\x03\x02\x02\
	\x02\u{1db}\u{1d9}\x03\x02\x02\x02\u{1db}\u{1dc}\x03\x02\x02\x02\u{1dc}\
	\u{1e2}\x03\x02\x02\x02\u{1dd}\u{1db}\x03\x02\x02\x02\u{1de}\u{1e3}\x05\
	\x20\x11\x02\u{1df}\u{1e3}\x05\x22\x12\x02\u{1e0}\u{1e3}\x05\x16\x0c\x02\
	\u{1e1}\u{1e3}\x07\u{f7}\x02\x02\u{1e2}\u{1de}\x03\x02\x02\x02\u{1e2}\u{1df}\
	\x03\x02\x02\x02\u{1e2}\u{1e0}\x03\x02\x02\x02\u{1e2}\u{1e1}\x03\x02\x02\
	\x02\u{1e3}\u{1e5}\x03\x02\x02\x02\u{1e4}\u{1db}\x03\x02\x02\x02\u{1e5}\
	\u{1e8}\x03\x02\x02\x02\u{1e6}\u{1e4}\x03\x02\x02\x02\u{1e6}\u{1e7}\x03\
	\x02\x02\x02\u{1e7}\u{1ec}\x03\x02\x02\x02\u{1e8}\u{1e6}\x03\x02\x02\x02\
	\u{1e9}\u{1eb}\x07\u{123}\x02\x02\u{1ea}\u{1e9}\x03\x02\x02\x02\u{1eb}\u{1ee}\
	\x03\x02\x02\x02\u{1ec}\u{1ea}\x03\x02\x02\x02\u{1ec}\u{1ed}\x03\x02\x02\
	\x02\u{1ed}\u{1ef}\x03\x02\x02\x02\u{1ee}\u{1ec}\x03\x02\x02\x02\u{1ef}\
	\u{1f0}\x07\u{11d}\x02\x02\u{1f0}\x11\x03\x02\x02\x02\u{1f1}\u{1f5}\x05\
	\x38\x1d\x02\u{1f2}\u{1f5}\x05\x36\x1c\x02\u{1f3}\u{1f5}\x05\x3a\x1e\x02\
	\u{1f4}\u{1f1}\x03\x02\x02\x02\u{1f4}\u{1f2}\x03\x02\x02\x02\u{1f4}\u{1f3}\
	\x03\x02\x02\x02\u{1f5}\u{1f8}\x03\x02\x02\x02\u{1f6}\u{1f7}\x09\x51\x02\
	\x02\u{1f7}\u{1f9}\x05\x14\x0b\x02\u{1f8}\u{1f6}\x03\x02\x02\x02\u{1f8}\
	\u{1f9}\x03\x02\x02\x02\u{1f9}\x13\x03\x02\x02\x02\u{1fa}\u{1fd}\x05\x38\
	\x1d\x02\u{1fb}\u{1fd}\x05\x36\x1c\x02\u{1fc}\u{1fa}\x03\x02\x02\x02\u{1fc}\
	\u{1fb}\x03\x02\x02\x02\u{1fd}\x15\x03\x02\x02\x02\u{1fe}\u{202}\x07\u{df}\
	\x02\x02\u{1ff}\u{201}\x07\u{123}\x02\x02\u{200}\u{1ff}\x03\x02\x02\x02\
	\u{201}\u{204}\x03\x02\x02\x02\u{202}\u{200}\x03\x02\x02\x02\u{202}\u{203}\
	\x03\x02\x02\x02\u{203}\u{205}\x03\x02\x02\x02\u{204}\u{202}\x03\x02\x02\
	\x02\u{205}\u{20e}\x05\x1e\x10\x02\u{206}\u{208}\x07\u{123}\x02\x02\u{207}\
	\u{206}\x03\x02\x02\x02\u{208}\u{209}\x03\x02\x02\x02\u{209}\u{207}\x03\
	\x02\x02\x02\u{209}\u{20a}\x03\x02\x02\x02\u{20a}\u{20b}\x03\x02\x02\x02\
	\u{20b}\u{20d}\x05\x1e\x10\x02\u{20c}\u{207}\x03\x02\x02\x02\u{20d}\u{210}\
	\x03\x02\x02\x02\u{20e}\u{20c}\x03\x02\x02\x02\u{20e}\u{20f}\x03\x02\x02\
	\x02\u{20f}\x17\x03\x02\x02\x02\u{210}\u{20e}\x03\x02\x02\x02\u{211}\u{216}\
	\x05\x1a\x0e\x02\u{212}\u{215}\x05\x10\x09\x02\u{213}\u{215}\x05\x30\x19\
	\x02\u{214}\u{212}\x03\x02\x02\x02\u{214}\u{213}\x03\x02\x02\x02\u{215}\
	\u{218}\x03\x02\x02\x02\u{216}\u{214}\x03\x02\x02\x02\u{216}\u{217}\x03\
	\x02\x02\x02\u{217}\u{21a}\x03\x02\x02\x02\u{218}\u{216}\x03\x02\x02\x02\
	\u{219}\u{21b}\x05\x1c\x0f\x02\u{21a}\u{219}\x03\x02\x02\x02\u{21a}\u{21b}\
	\x03\x02\x02\x02\u{21b}\x19\x03\x02\x02\x02\u{21c}\u{224}\x07\u{de}\x02\
	\x02\u{21d}\u{21f}\x07\u{123}\x02\x02\u{21e}\u{21d}\x03\x02\x02\x02\u{21f}\
	\u{222}\x03\x02\x02\x02\u{220}\u{21e}\x03\x02\x02\x02\u{220}\u{221}\x03\
	\x02\x02\x02\u{221}\u{223}\x03\x02\x02\x02\u{222}\u{220}\x03\x02\x02\x02\
	\u{223}\u{225}\x05\x1e\x10\x02\u{224}\u{220}\x03\x02\x02\x02\u{225}\u{226}\
	\x03\x02\x02\x02\u{226}\u{224}\x03\x02\x02\x02\u{226}\u{227}\x03\x02\x02\
	\x02\u{227}\u{228}\x03\x02\x02\x02\u{228}\u{229}\x07\u{11d}\x02\x02\u{229}\
	\x1b\x03\x02\x02\x02\u{22a}\u{22b}\x07\u{de}\x02\x02\u{22b}\u{22c}\x07\u{ca}\
	\x02\x02\u{22c}\u{22d}\x07\u{11d}\x02\x02\u{22d}\x1d\x03\x02\x02\x02\u{22e}\
	\u{232}\x05\x24\x13\x02\u{22f}\u{231}\x07\u{123}\x02\x02\u{230}\u{22f}\x03\
	\x02\x02\x02\u{231}\u{234}\x03\x02\x02\x02\u{232}\u{230}\x03\x02\x02\x02\
	\u{232}\u{233}\x03\x02\x02\x02\u{233}\u{235}\x03\x02\x02\x02\u{234}\u{232}\
	\x03\x02\x02\x02\u{235}\u{239}\x07\x11\x02\x02\u{236}\u{238}\x07\u{123}\
	\x02\x02\u{237}\u{236}\x03\x02\x02\x02\u{238}\u{23b}\x03\x02\x02\x02\u{239}\
	\u{237}\x03\x02\x02\x02\u{239}\u{23a}\x03\x02\x02\x02\u{23a}\u{23c}\x03\
	\x02\x02\x02\u{23b}\u{239}\x03\x02\x02\x02\u{23c}\u{244}\x05\x26\x14\x02\
	\u{23d}\u{23f}\x07\u{123}\x02\x02\u{23e}\u{23d}\x03\x02\x02\x02\u{23f}\u{242}\
	\x03\x02\x02\x02\u{240}\u{23e}\x03\x02\x02\x02\u{240}\u{241}\x03\x02\x02\
	\x02\u{241}\u{243}\x03\x02\x02\x02\u{242}\u{240}\x03\x02\x02\x02\u{243}\
	\u{245}\x05\x20\x11\x02\u{244}\u{240}\x03\x02\x02\x02\u{244}\u{245}\x03\
	\x02\x02\x02\u{245}\u{24d}\x03\x02\x02\x02\u{246}\u{248}\x07\u{123}\x02\
	\x02\u{247}\u{246}\x03\x02\x02\x02\u{248}\u{24b}\x03\x02\x02\x02\u{249}\
	\u{247}\x03\x02\x02\x02\u{249}\u{24a}\x03\x02\x02\x02\u{24a}\u{24c}\x03\
	\x02\x02\x02\u{24b}\u{249}\x03\x02\x02\x02\u{24c}\u{24e}\x05\x22\x12\x02\
	\u{24d}\u{249}\x03\x02\x02\x02\u{24d}\u{24e}\x03\x02\x02\x02\u{24e}\x1f\
	\x03\x02\x02\x02\u{24f}\u{253}\x09\x51\x02\x02\u{250}\u{252}\x07\u{123}\
	\x02\x02\u{251}\u{250}\x03\x02\x02\x02\u{252}\u{255}\x03\x02\x02\x02\u{253}\
	\u{251}\x03\x02\x02\x02\u{253}\u{254}\x03\x02\x02\x02\u{254}\u{258}\x03\
	\x02\x02\x02\u{255}\u{253}\x03\x02\x02\x02\u{256}\u{259}\x05\x38\x1d\x02\
	\u{257}\u{259}\x05\x36\x1c\x02\u{258}\u{256}\x03\x02\x02\x02\u{258}\u{257}\
	\x03\x02\x02\x02\u{259}\x21\x03\x02\x02\x02\u{25a}\u{25e}\x07\u{cc}\x02\
	\x02\u{25b}\u{25d}\x07\u{123}\x02\x02\u{25c}\u{25b}\x03\x02\x02\x02\u{25d}\
	\u{260}\x03\x02\x02\x02\u{25e}\u{25c}\x03\x02\x02\x02\u{25e}\u{25f}\x03\
	\x02\x02\x02\u{25f}\u{263}\x03\x02\x02\x02\u{260}\u{25e}\x03\x02\x02\x02\
	\u{261}\u{264}\x05\x38\x1d\x02\u{262}\u{264}\x05\x36\x1c\x02\u{263}\u{261}\
	\x03\x02\x02\x02\u{263}\u{262}\x03\x02\x02\x02\u{264}\x23\x03\x02\x02\x02\
	\u{265}\u{26a}\x05\x38\x1d\x02\u{266}\u{26a}\x05\x36\x1c\x02\u{267}\u{26a}\
	\x05\x2e\x18\x02\u{268}\u{26a}\x05\x34\x1b\x02\u{269}\u{265}\x03\x02\x02\
	\x02\u{269}\u{266}\x03\x02\x02\x02\u{269}\u{267}\x03\x02\x02\x02\u{269}\
	\u{268}\x03\x02\x02\x02\u{26a}\x25\x03\x02\x02\x02\u{26b}\u{270}\x05\x38\
	\x1d\x02\u{26c}\u{270}\x05\x36\x1c\x02\u{26d}\u{270}\x05\x2e\x18\x02\u{26e}\
	\u{270}\x05\x34\x1b\x02\u{26f}\u{26b}\x03\x02\x02\x02\u{26f}\u{26c}\x03\
	\x02\x02\x02\u{26f}\u{26d}\x03\x02\x02\x02\u{26f}\u{26e}\x03\x02\x02\x02\
	\u{270}\x27\x03\x02\x02\x02\u{271}\u{273}\x07\x32\x02\x02\u{272}\u{274}\
	\x07\u{11d}\x02\x02\u{273}\u{272}\x03\x02\x02\x02\u{273}\u{274}\x03\x02\
	\x02\x02\u{274}\x29\x03\x02\x02\x02\u{275}\u{277}\x09\x52\x02\x02\u{276}\
	\u{278}\x07\u{11d}\x02\x02\u{277}\u{276}\x03\x02\x02\x02\u{277}\u{278}\x03\
	\x02\x02\x02\u{278}\x2b\x03\x02\x02\x02\u{279}\u{27a}\x07\u{fe}\x02\x02\
	\u{27a}\u{27c}\x05\x38\x1d\x02\u{27b}\u{27d}\x07\u{11d}\x02\x02\u{27c}\u{27b}\
	\x03\x02\x02\x02\u{27c}\u{27d}\x03\x02\x02\x02\u{27d}\x2d\x03\x02\x02\x02\
	\u{27e}\u{280}\x07\u{11e}\x02\x02\u{27f}\u{281}\x05\x30\x19\x02\u{280}\u{27f}\
	\x03\x02\x02\x02\u{280}\u{281}\x03\x02\x02\x02\u{281}\u{282}\x03\x02\x02\
	\x02\u{282}\u{283}\x07\u{11e}\x02\x02\u{283}\x2f\x03\x02\x02\x02\u{284}\
	\u{287}\x05\x34\x1b\x02\u{285}\u{287}\x07\u{123}\x02\x02\u{286}\u{284}\x03\
	\x02\x02\x02\u{286}\u{285}\x03\x02\x02\x02\u{287}\u{288}\x03\x02\x02\x02\
	\u{288}\u{286}\x03\x02\x02\x02\u{288}\u{289}\x03\x02\x02\x02\u{289}\x31\
	\x03\x02\x02\x02\u{28a}\u{28f}\x05\x34\x1b\x02\u{28b}\u{28f}\x07\x1b\x02\
	\x02\u{28c}\u{28f}\x07\u{de}\x02\x02\u{28d}\u{28f}\x07\u{123}\x02\x02\u{28e}\
	\u{28a}\x03\x02\x02\x02\u{28e}\u{28b}\x03\x02\x02\x02\u{28e}\u{28c}\x03\
	\x02\x02\x02\u{28e}\u{28d}\x03\x02\x02\x02\u{28f}\u{290}\x03\x02\x02\x02\
	\u{290}\u{28e}\x03\x02\x02\x02\u{290}\u{291}\x03\x02\x02\x02\u{291}\x33\
	\x03\x02\x02\x02\u{292}\u{29a}\x05\x36\x1c\x02\u{293}\u{29a}\x05\x38\x1d\
	\x02\u{294}\u{29a}\x05\x3a\x1e\x02\u{295}\u{29a}\x07\u{126}\x02\x02\u{296}\
	\u{29a}\x07\u{11d}\x02\x02\u{297}\u{29a}\x07\x5a\x02\x02\u{298}\u{29a}\x07\
	\u{e1}\x02\x02\u{299}\u{292}\x03\x02\x02\x02\u{299}\u{293}\x03\x02\x02\x02\
	\u{299}\u{294}\x03\x02\x02\x02\u{299}\u{295}\x03\x02\x02\x02\u{299}\u{296}\
	\x03\x02\x02\x02\u{299}\u{297}\x03\x02\x02\x02\u{299}\u{298}\x03\x02\x02\
	\x02\u{29a}\u{29b}\x03\x02\x02\x02\u{29b}\u{299}\x03\x02\x02\x02\u{29b}\
	\u{29c}\x03\x02\x02\x02\u{29c}\x35\x03\x02\x02\x02\u{29d}\u{2a0}\x07\u{121}\
	\x02\x02\u{29e}\u{2a0}\x05\x3c\x1f\x02\u{29f}\u{29d}\x03\x02\x02\x02\u{29f}\
	\u{29e}\x03\x02\x02\x02\u{2a0}\x37\x03\x02\x02\x02\u{2a1}\u{2a2}\x09\x53\
	\x02\x02\u{2a2}\x39\x03\x02\x02\x02\u{2a3}\u{2a4}\x07\u{122}\x02\x02\u{2a4}\
	\x3b\x03\x02\x02\x02\u{2a5}\u{2a6}\x09\x54\x02\x02\u{2a6}\x3d\x03\x02\x02\
	\x02\x4c\x4a\x4c\x53\x57\x59\x5f\x64\x7d\u{96}\u{99}\u{9c}\u{9f}\u{b3}\u{bb}\
	\u{db}\u{e3}\u{e9}\u{fa}\u{143}\u{152}\u{168}\u{172}\u{178}\u{188}\u{191}\
	\u{194}\u{197}\u{19a}\u{19d}\u{1a0}\u{1b4}\u{1b7}\u{1bf}\u{1c6}\u{1cd}\u{1d4}\
	\u{1db}\u{1e2}\u{1e6}\u{1ec}\u{1f4}\u{1f8}\u{1fc}\u{202}\u{209}\u{20e}\u{214}\
	\u{216}\u{21a}\u{220}\u{226}\u{232}\u{239}\u{240}\u{244}\u{249}\u{24d}\u{253}\
	\u{258}\u{25e}\u{263}\u{269}\u{26f}\u{273}\u{277}\u{27c}\u{280}\u{286}\u{288}\
	\u{28e}\u{290}\u{299}\u{29b}\u{29f}";

