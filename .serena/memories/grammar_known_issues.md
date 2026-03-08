# ANTLR Grammar Known Issues

## ROUNDED Not in cobolWord Rule
**Status**: Known, deferred
**Impact**: Field names containing "ROUNDED" (e.g., WS-ROUNDED) break parsing
**Root Cause**: ROUNDED keyword (Cobol85.g4 line 4906) is NOT in the `cobolWord` rule (lines 3041-3167). 
The lexer tokenizes WS-ROUNDED as WS- + ROUNDED(keyword), breaking the identifier.
**Affected**: `ADD x TO WS-ROUNDED ROUNDED`, `MULTIPLY x BY WS-ROUNDED ROUNDED`, `DIVIDE x BY y GIVING WS-ROUNDED ROUNDED`
**Fix**: Add ROUNDED to cobolWord alternatives in grammar/Cobol85.g4 and regenerate parser using scripts/generate_parser.sh
**Workaround**: extract_data_ref_from_identifier_text() in proc_listener.rs strips "ROUNDED" from text, but this causes empty field names when the entire identifier IS "ROUNDED" suffixed.
**Programs affected**: arithmetic-verbs_test (3 errors: ws_ empty field name)

## Other Keywords Potentially Missing from cobolWord
Any COBOL reserved word not listed in cobolWord (lines 3041-3167) will cause the same issue if used as part of a data name. Need audit when grammar is next regenerated.
