//! Internal module providing handy function

use std::convert::TryInto;

macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}

/// Converts a &[u8] into an iterator of `u32`s
pub fn to_u32(s: &[u8]) -> impl ExactSizeIterator<Item = u32> + '_ {
    assert_eq!(s.len() % 4, 0);
    s.chunks_exact(4)
        .map(|data| u32::from_le_bytes(data.try_into().unwrap()))
}

#[inline]
pub fn read_u32(s: &[u8]) -> u32 {
    u32::from_le_bytes(s[..4].try_into().unwrap())
}

#[inline]
pub fn read_i32(s: &[u8]) -> i32 {
    i32::from_le_bytes(s[..4].try_into().unwrap())
}

#[inline]
pub fn read_u16(s: &[u8]) -> u16 {
    u16::from_le_bytes(s[..2].try_into().unwrap())
}

#[inline]
pub fn read_u64(s: &[u8]) -> u64 {
    u64::from_le_bytes(s[..8].try_into().unwrap())
}

#[inline]
pub fn read_usize(s: &[u8]) -> usize {
    read_u32(s).try_into().unwrap()
}

#[inline]
pub fn read_f64(s: &[u8]) -> f64 {
    f64::from_le_bytes(s[..8].try_into().unwrap())
}

/// Push literal column into a String buffer
pub fn push_column(mut col: u32, buf: &mut String) {
    if col < 26 {
        buf.push((b'A' + col as u8) as char);
    } else {
        let mut rev = String::new();
        while col >= 26 {
            let c = col % 26;
            rev.push((b'A' + c as u8) as char);
            col -= c;
            col /= 26;
        }
        buf.extend(rev.chars().rev());
    }
}

pub const FTAB_LEN: usize = 485;

/* [MS-XLS] 2.5.198.17 */
/* [MS-XLSB] 2.5.97.10 */
pub const FTAB: [&str; FTAB_LEN] = [
    "COUNT",
    "IF",
    "ISNA",
    "ISERROR",
    "SUM",
    "AVERAGE",
    "MIN",
    "MAX",
    "ROW",
    "COLUMN",
    "NA",
    "NPV",
    "STDEV",
    "DOLLAR",
    "FIXED",
    "SIN",
    "COS",
    "TAN",
    "ATAN",
    "PI",
    "SQRT",
    "EXP",
    "LN",
    "LOG10",
    "ABS",
    "INT",
    "SIGN",
    "ROUND",
    "LOOKUP",
    "INDEX",
    "REPT",
    "MID",
    "LEN",
    "VALUE",
    "TRUE",
    "FALSE",
    "AND",
    "OR",
    "NOT",
    "MOD",
    "DCOUNT",
    "DSUM",
    "DAVERAGE",
    "DMIN",
    "DMAX",
    "DSTDEV",
    "VAR",
    "DVAR",
    "TEXT",
    "LINEST",
    "TREND",
    "LOGEST",
    "GROWTH",
    "GOTO",
    "HALT",
    "RETURN",
    "PV",
    "FV",
    "NPER",
    "PMT",
    "RATE",
    "MIRR",
    "IRR",
    "RAND",
    "MATCH",
    "DATE",
    "TIME",
    "DAY",
    "MONTH",
    "YEAR",
    "WEEKDAY",
    "HOUR",
    "MINUTE",
    "SECOND",
    "NOW",
    "AREAS",
    "ROWS",
    "COLUMNS",
    "OFFSET",
    "ABSREF",
    "RELREF",
    "ARGUMENT",
    "SEARCH",
    "TRANSPOSE",
    "ERROR",
    "STEP",
    "TYPE",
    "ECHO",
    "SET.NAME",
    "CALLER",
    "DEREF",
    "WINDOWS",
    "SERIES",
    "DOCUMENTS",
    "ACTIVE.CELL",
    "SELECTION",
    "RESULT",
    "ATAN2",
    "ASIN",
    "ACOS",
    "CHOOSE",
    "HLOOKUP",
    "VLOOKUP",
    "LINKS",
    "INPUT",
    "ISREF",
    "GET.FORMULA",
    "GET.NAME",
    "SET.VALUE",
    "LOG",
    "EXEC",
    "CHAR",
    "LOWER",
    "UPPER",
    "PROPER",
    "LEFT",
    "RIGHT",
    "EXACT",
    "TRIM",
    "REPLACE",
    "SUBSTITUTE",
    "CODE",
    "NAMES",
    "DIRECTORY",
    "FIND",
    "CELL",
    "ISERR",
    "ISTEXT",
    "ISNUMBER",
    "ISBLANK",
    "T",
    "N",
    "FOPEN",
    "FCLOSE",
    "FSIZE",
    "FREADLN",
    "FREAD",
    "FWRITELN",
    "FWRITE",
    "FPOS",
    "DATEVALUE",
    "TIMEVALUE",
    "SLN",
    "SYD",
    "DDB",
    "GET.DEF",
    "REFTEXT",
    "TEXTREF",
    "INDIRECT",
    "REGISTER",
    "CALL",
    "ADD.BAR",
    "ADD.MENU",
    "ADD.COMMAND",
    "ENABLE.COMMAND",
    "CHECK.COMMAND",
    "RENAME.COMMAND",
    "SHOW.BAR",
    "DELETE.MENU",
    "DELETE.COMMAND",
    "GET.CHART.ITEM",
    "DIALOG.BOX",
    "CLEAN",
    "MDETERM",
    "MINVERSE",
    "MMULT",
    "FILES",
    "IPMT",
    "PPMT",
    "COUNTA",
    "CANCEL.KEY",
    "FOR",
    "WHILE",
    "BREAK",
    "NEXT",
    "INITIATE",
    "REQUEST",
    "POKE",
    "EXECUTE",
    "TERMINATE",
    "RESTART",
    "HELP",
    "GET.BAR",
    "PRODUCT",
    "FACT",
    "GET.CELL",
    "GET.WORKSPACE",
    "GET.WINDOW",
    "GET.DOCUMENT",
    "DPRODUCT",
    "ISNONTEXT",
    "GET.NOTE",
    "NOTE",
    "STDEVP",
    "VARP",
    "DSTDEVP",
    "DVARP",
    "TRUNC",
    "ISLOGICAL",
    "DCOUNTA",
    "DELETE.BAR",
    "UNREGISTER",
    "",
    "",
    "USDOLLAR",
    "FINDB",
    "SEARCHB",
    "REPLACEB",
    "LEFTB",
    "RIGHTB",
    "MIDB",
    "LENB",
    "ROUNDUP",
    "ROUNDDOWN",
    "ASC",
    "DBCS",
    "RANK",
    "",
    "",
    "ADDRESS",
    "DAYS360",
    "TODAY",
    "VDB",
    "ELSE",
    "ELSE.IF",
    "END.IF",
    "FOR.CELL",
    "MEDIAN",
    "SUMPRODUCT",
    "SINH",
    "COSH",
    "TANH",
    "ASINH",
    "ACOSH",
    "ATANH",
    "DGET",
    "CREATE.OBJECT",
    "VOLATILE",
    "LAST.ERROR",
    "CUSTOM.UNDO",
    "CUSTOM.REPEAT",
    "FORMULA.CONVERT",
    "GET.LINK.INFO",
    "TEXT.BOX",
    "INFO",
    "GROUP",
    "GET.OBJECT",
    "DB",
    "PAUSE",
    "",
    "",
    "RESUME",
    "FREQUENCY",
    "ADD.TOOLBAR",
    "DELETE.TOOLBAR",
    "User",
    "RESET.TOOLBAR",
    "EVALUATE",
    "GET.TOOLBAR",
    "GET.TOOL",
    "SPELLING.CHECK",
    "ERROR.TYPE",
    "APP.TITLE",
    "WINDOW.TITLE",
    "SAVE.TOOLBAR",
    "ENABLE.TOOL",
    "PRESS.TOOL",
    "REGISTER.ID",
    "GET.WORKBOOK",
    "AVEDEV",
    "BETADIST",
    "GAMMALN",
    "BETAINV",
    "BINOMDIST",
    "CHIDIST",
    "CHIINV",
    "COMBIN",
    "CONFIDENCE",
    "CRITBINOM",
    "EVEN",
    "EXPONDIST",
    "FDIST",
    "FINV",
    "FISHER",
    "FISHERINV",
    "FLOOR",
    "GAMMADIST",
    "GAMMAINV",
    "CEILING",
    "HYPGEOMDIST",
    "LOGNORMDIST",
    "LOGINV",
    "NEGBINOMDIST",
    "NORMDIST",
    "NORMSDIST",
    "NORMINV",
    "NORMSINV",
    "STANDARDIZE",
    "ODD",
    "PERMUT",
    "POISSON",
    "TDIST",
    "WEIBULL",
    "SUMXMY2",
    "SUMX2MY2",
    "SUMX2PY2",
    "CHITEST",
    "CORREL",
    "COVAR",
    "FORECAST",
    "FTEST",
    "INTERCEPT",
    "PEARSON",
    "RSQ",
    "STEYX",
    "SLOPE",
    "TTEST",
    "PROB",
    "DEVSQ",
    "GEOMEAN",
    "HARMEAN",
    "SUMSQ",
    "KURT",
    "SKEW",
    "ZTEST",
    "LARGE",
    "SMALL",
    "QUARTILE",
    "PERCENTILE",
    "PERCENTRANK",
    "MODE",
    "TRIMMEAN",
    "TINV",
    "",
    "MOVIE.COMMAND",
    "GET.MOVIE",
    "CONCATENATE",
    "POWER",
    "PIVOT.ADD.DATA",
    "GET.PIVOT.TABLE",
    "GET.PIVOT.FIELD",
    "GET.PIVOT.ITEM",
    "RADIANS",
    "DEGREES",
    "SUBTOTAL",
    "SUMIF",
    "COUNTIF",
    "COUNTBLANK",
    "SCENARIO.GET",
    "OPTIONS.LISTS.GET",
    "ISPMT",
    "DATEDIF",
    "DATESTRING",
    "NUMBERSTRING",
    "ROMAN",
    "OPEN.DIALOG",
    "SAVE.DIALOG",
    "VIEW.GET",
    "GETPIVOTDATA",
    "HYPERLINK",
    "PHONETIC",
    "AVERAGEA",
    "MAXA",
    "MINA",
    "STDEVPA",
    "VARPA",
    "STDEVA",
    "VARA",
    "BAHTTEXT",
    "THAIDAYOFWEEK",
    "THAIDIGIT",
    "THAIMONTHOFYEAR",
    "THAINUMSOUND",
    "THAINUMSTRING",
    "THAISTRINGLENGTH",
    "ISTHAIDIGIT",
    "ROUNDBAHTDOWN",
    "ROUNDBAHTUP",
    "THAIYEAR",
    "RTD",
    "CUBEVALUE",
    "CUBEMEMBER",
    "CUBEMEMBERPROPERTY",
    "CUBERANKEDMEMBER",
    "HEX2BIN",
    "HEX2DEC",
    "HEX2OCT",
    "DEC2BIN",
    "DEC2HEX",
    "DEC2OCT",
    "OCT2BIN",
    "OCT2HEX",
    "OCT2DEC",
    "BIN2DEC",
    "BIN2OCT",
    "BIN2HEX",
    "IMSUB",
    "IMDIV",
    "IMPOWER",
    "IMABS",
    "IMSQRT",
    "IMLN",
    "IMLOG2",
    "IMLOG10",
    "IMSIN",
    "IMCOS",
    "IMEXP",
    "IMARGUMENT",
    "IMCONJUGATE",
    "IMAGINARY",
    "IMREAL",
    "COMPLEX",
    "IMSUM",
    "IMPRODUCT",
    "SERIESSUM",
    "FACTDOUBLE",
    "SQRTPI",
    "QUOTIENT",
    "DELTA",
    "GESTEP",
    "ISEVEN",
    "ISODD",
    "MROUND",
    "ERF",
    "ERFC",
    "BESSELJ",
    "BESSELK",
    "BESSELY",
    "BESSELI",
    "XIRR",
    "XNPV",
    "PRICEMAT",
    "YIELDMAT",
    "INTRATE",
    "RECEIVED",
    "DISC",
    "PRICEDISC",
    "YIELDDISC",
    "TBILLEQ",
    "TBILLPRICE",
    "TBILLYIELD",
    "PRICE",
    "YIELD",
    "DOLLARDE",
    "DOLLARFR",
    "NOMINAL",
    "EFFECT",
    "CUMPRINC",
    "CUMIPMT",
    "EDATE",
    "EOMONTH",
    "YEARFRAC",
    "COUPDAYBS",
    "COUPDAYS",
    "COUPDAYSNC",
    "COUPNCD",
    "COUPNUM",
    "COUPPCD",
    "DURATION",
    "MDURATION",
    "ODDLPRICE",
    "ODDLYIELD",
    "ODDFPRICE",
    "ODDFYIELD",
    "RANDBETWEEN",
    "WEEKNUM",
    "AMORDEGRC",
    "AMORLINC",
    "CONVERT",
    //     "SHEETJS",
    "ACCRINT",
    "ACCRINTM",
    "WORKDAY",
    "NETWORKDAYS",
    "GCD",
    "MULTINOMIAL",
    "LCM",
    "FVSCHEDULE",
    "CUBEKPIMEMBER",
    "CUBESET",
    "CUBESETCOUNT",
    "IFERROR",
    "COUNTIFS",
    "SUMIFS",
    "AVERAGEIF",
    "AVERAGEIFS",
];

pub const FTAB_ARGC: [u8; FTAB_LEN] = [
    255, // "COUNT",
    3,   // "IF",
    1,   // "ISNA",
    1,   // "ISERROR",
    255, // "SUM",
    255, // "AVERAGE",
    255, // "MIN",
    255, // "MAX",
    1,   // "ROW",
    1,   // "COLUMN",
    0,   // "NA",
    254, // "NPV",
    255, // "STDEV",
    2,   // "DOLLAR",
    3,   // "FIXED",
    1,   // "SIN",
    1,   // "COS",
    1,   // "TAN",
    1,   // "ATAN",
    0,   // "PI",
    1,   // "SQRT",
    1,   // "EXP",
    1,   // "LN",
    1,   // "LOG10",
    1,   // "ABS",
    1,   // "INT",
    1,   // "SIGN",
    2,   // "ROUND",
    3,   // "LOOKUP",
    4,   // "INDEX",
    2,   // "REPT",
    3,   // "MID",
    1,   // "LEN",
    1,   // "VALUE",
    0,   // "TRUE",
    0,   // "FALSE",
    255, // "AND",
    255, // "OR",
    1,   // "NOT",
    2,   // "MOD",
    3,   // "DCOUNT",
    3,   // "DSUM",
    3,   // "DAVERAGE",
    3,   // "DMIN",
    3,   // "DMAX",
    3,   // "DSTDEV",
    255, // "VAR",
    3,   // "DVAR",
    2,   // "TEXT",
    4,   // "LINEST",
    4,   // "TREND",
    4,   // "LOGEST",
    4,   // "GROWTH",
    1,   // "GOTO",
    1,   // "HALT",
    1,   // "RETURN",
    5,   // "PV",
    5,   // "FV",
    5,   // "NPER",
    5,   // "PMT",
    6,   // "RATE",
    3,   // "MIRR",
    2,   // "IRR",
    0,   // "RAND",
    3,   // "MATCH",
    3,   // "DATE",
    3,   // "TIME",
    1,   // "DAY",
    1,   // "MONTH",
    1,   // "YEAR",
    2,   // "WEEKDAY",
    1,   // "HOUR",
    1,   // "MINUTE",
    1,   // "SECOND",
    0,   // "NOW",
    1,   // "AREAS",
    1,   // "ROWS",
    1,   // "COLUMNS",
    5,   // "OFFSET",
    2,   // "ABSREF",
    2,   // "RELREF",
    3,   // "ARGUMENT",
    3,   // "SEARCH",
    1,   // "TRANSPOSE",
    2,   // "ERROR",
    0,   // "STEP",
    1,   // "TYPE",
    1,   // "ECHO",
    2,   // "SET.NAME",
    0,   // "CALLER",
    1,   // "DEREF",
    2,   // "WINDOWS",
    2,   // "SERIES",
    2,   // "DOCUMENTS",
    0,   // "ACTIVE.CELL",
    0,   // "SELECTION",
    1,   // "RESULT",
    2,   // "ATAN2",
    1,   // "ASIN",
    1,   // "ACOS",
    255, // "CHOOSE",
    4,   // "HLOOKUP",
    4,   // "VLOOKUP",
    2,   // "LINKS",
    7,   // "INPUT",
    1,   // "ISREF",
    1,   // "GET.FORMULA",
    2,   // "GET.NAME",
    2,   // "SET.VALUE",
    2,   // "LOG",
    4,   // "EXEC",
    1,   // "CHAR",
    1,   // "LOWER",
    1,   // "UPPER",
    1,   // "PROPER",
    2,   // "LEFT",
    2,   // "RIGHT",
    2,   // "EXACT",
    1,   // "TRIM",
    4,   // "REPLACE",
    4,   // "SUBSTITUTE",
    1,   // "CODE",
    3,   // "NAMES",
    1,   // "DIRECTORY",
    3,   // "FIND",
    2,   // "CELL",
    1,   // "ISERR",
    1,   // "ISTEXT",
    1,   // "ISNUMBER",
    1,   // "ISBLANK",
    1,   // "T",
    1,   // "N",
    2,   // "FOPEN",
    1,   // "FCLOSE",
    1,   // "FSIZE",
    1,   // "FREADLN",
    2,   // "FREAD",
    2,   // "FWRITELN",
    2,   // "FWRITE",
    2,   // "FPOS",
    1,   // "DATEVALUE",
    1,   // "TIMEVALUE",
    3,   // "SLN",
    4,   // "SYD",
    5,   // "DDB",
    3,   // "GET.DEF",
    2,   // "REFTEXT",
    2,   // "TEXTREF",
    2,   // "INDIRECT",
    255, // "REGISTER",
    255, // "CALL",
    1,   // "ADD.BAR",
    4,   // "ADD.MENU",
    5,   // "ADD.COMMAND",
    5,   // "ENABLE.COMMAND",
    5,   // "CHECK.COMMAND",
    5,   // "RENAME.COMMAND",
    1,   // "SHOW.BAR",
    3,   // "DELETE.MENU",
    4,   // "DELETE.COMMAND",
    3,   // "GET.CHART.ITEM",
    1,   // "DIALOG.BOX",
    1,   // "CLEAN",
    1,   // "MDETERM",
    1,   // "MINVERSE",
    1,   // "MMULT",
    2,   // "FILES",
    6,   // "IPMT",
    6,   // "PPMT",
    255, // "COUNTA",
    2,   // "CANCEL.KEY",
    4,   // "FOR",
    1,   // "WHILE",
    0,   // "BREAK",
    0,   // "NEXT",
    2,   // "INITIATE",
    2,   // "REQUEST",
    3,   // "POKE",
    2,   // "EXECUTE",
    1,   // "TERMINATE",
    1,   // "RESTART",
    1,   // "HELP",
    4,   // "GET.BAR",
    255, // "PRODUCT",
    1,   // "FACT",
    2,   // "GET.CELL",
    1,   // "GET.WORKSPACE",
    2,   // "GET.WINDOW",
    2,   // "GET.DOCUMENT",
    3,   // "DPRODUCT",
    1,   // "ISNONTEXT",
    3,   // "GET.NOTE",
    4,   // "NOTE",
    255, // "STDEVP",
    255, // "VARP",
    3,   // "DSTDEVP",
    3,   // "DVARP",
    2,   // "TRUNC",
    1,   // "ISLOGICAL",
    3,   // "DCOUNTA",
    1,   // "DELETE.BAR",
    1,   // "UNREGISTER",
    0,   // "",
    0,   // "",
    2,   // "USDOLLAR",
    3,   // "FINDB",
    3,   // "SEARCHB",
    4,   // "REPLACEB",
    2,   // "LEFTB",
    2,   // "RIGHTB",
    3,   // "MIDB",
    3,   // "LENB",
    2,   // "ROUNDUP",
    2,   // "ROUNDDOWN",
    1,   // "ASC",
    1,   // "DBCS",
    3,   // "RANK",
    0,   // "",
    0,   // "",
    5,   // "ADDRESS",
    3,   // "DAYS360",
    0,   // "TODAY",
    7,   // "VDB",
    0,   // "ELSE",
    1,   // "ELSE.IF",
    0,   // "END.IF",
    3,   // "FOR.CELL",
    255, // "MEDIAN",
    255, // "SUMPRODUCT",
    1,   // "SINH",
    1,   // "COSH",
    1,   // "TANH",
    1,   // "ASINH",
    1,   // "ACOSH",
    1,   // "ATANH",
    3,   // "DGET",
    11,  // "CREATE.OBJECT",
    1,   // "VOLATILE",
    0,   // "LAST.ERROR",
    2,   // "CUSTOM.UNDO",
    3,   // "CUSTOM.REPEAT",
    5,   // "FORMULA.CONVERT",
    4,   // "GET.LINK.INFO",
    4,   // "TEXT.BOX",
    1,   // "INFO",
    0,   // "GROUP",
    5,   // "GET.OBJECT",
    5,   // "DB",
    1,   // "PAUSE",
    0,   // "",
    0,   // "",
    1,   // "RESUME",
    2,   // "FREQUENCY",
    2,   // "ADD.TOOLBAR",
    1,   // "DELETE.TOOLBAR",
    255, // "User",
    1,   // "RESET.TOOLBAR",
    1,   // "EVALUATE",
    2,   // "GET.TOOLBAR",
    3,   // "GET.TOOL",
    3,   // "SPELLING.CHECK",
    1,   // "ERROR.TYPE",
    1,   // "APP.TITLE",
    1,   // "WINDOW.TITLE",
    2,   // "SAVE.TOOLBAR",
    3,   // "ENABLE.TOOL",
    3,   // "PRESS.TOOL",
    3,   // "REGISTER.ID",
    2,   // "GET.WORKBOOK",
    255, // "AVEDEV",
    5,   // "BETADIST",
    1,   // "GAMMALN",
    5,   // "BETAINV",
    4,   // "BINOMDIST",
    2,   // "CHIDIST",
    2,   // "CHIINV",
    2,   // "COMBIN",
    3,   // "CONFIDENCE",
    3,   // "CRITBINOM",
    1,   // "EVEN",
    3,   // "EXPONDIST",
    3,   // "FDIST",
    3,   // "FINV",
    1,   // "FISHER",
    1,   // "FISHERINV",
    2,   // "FLOOR",
    4,   // "GAMMADIST",
    3,   // "GAMMAINV",
    2,   // "CEILING",
    4,   // "HYPGEOMDIST",
    3,   // "LOGNORMDIST",
    3,   // "LOGINV",
    3,   // "NEGBINOMDIST",
    4,   // "NORMDIST",
    1,   // "NORMSDIST",
    3,   // "NORMINV",
    1,   // "NORMSINV",
    3,   // "STANDARDIZE",
    1,   // "ODD",
    2,   // "PERMUT",
    3,   // "POISSON",
    3,   // "TDIST",
    4,   // "WEIBULL",
    2,   // "SUMXMY2",
    2,   // "SUMX2MY2",
    2,   // "SUMX2PY2",
    2,   // "CHITEST",
    2,   // "CORREL",
    2,   // "COVAR",
    3,   // "FORECAST",
    2,   // "FTEST",
    2,   // "INTERCEPT",
    2,   // "PEARSON",
    2,   // "RSQ",
    2,   // "STEYX",
    2,   // "SLOPE",
    4,   // "TTEST",
    4,   // "PROB",
    255, // "DEVSQ",
    255, // "GEOMEAN",
    255, // "HARMEAN",
    255, // "SUMSQ",
    255, // "KURT",
    255, // "SKEW",
    3,   // "ZTEST",
    2,   // "LARGE",
    2,   // "SMALL",
    2,   // "QUARTILE",
    2,   // "PERCENTILE",
    3,   // "PERCENTRANK",
    255, // "MODE",
    2,   // "TRIMMEAN",
    2,   // "TINV",
    4,   // "",
    4,   // "MOVIE.COMMAND",
    3,   // "GET.MOVIE",
    255, // "CONCATENATE",
    2,   // "POWER",
    9,   // "PIVOT.ADD.DATA",
    2,   // "GET.PIVOT.TABLE",
    3,   // "GET.PIVOT.FIELD",
    4,   // "GET.PIVOT.ITEM",
    1,   // "RADIANS",
    1,   // "DEGREES",
    255, // "SUBTOTAL",
    3,   // "SUMIF",
    2,   // "COUNTIF",
    1,   // "COUNTBLANK",
    2,   // "SCENARIO.GET",
    1,   // "OPTIONS.LISTS.GET",
    4,   // "ISPMT",
    3,   // "DATEDIF",
    1,   // "DATESTRING",
    2,   // "NUMBERSTRING",
    2,   // "ROMAN",
    4,   // "OPEN.DIALOG",
    5,   // "SAVE.DIALOG",
    2,   // "VIEW.GET",
    128, // "GETPIVOTDATA",
    2,   // "HYPERLINK",
    1,   // "PHONETIC",
    255, // "AVERAGEA",
    255, // "MAXA",
    255, // "MINA",
    255, // "STDEVPA",
    255, // "VARPA",
    255, // "STDEVA",
    255, // "VARA",
    1,   // "BAHTTEXT",
    1,   // "THAIDAYOFWEEK",
    1,   // "THAIDIGIT",
    1,   // "THAIMONTHOFYEAR",
    1,   // "THAINUMSOUND",
    1,   // "THAINUMSTRING",
    1,   // "THAISTRINGLENGTH",
    1,   // "ISTHAIDIGIT",
    1,   // "ROUNDBAHTDOWN",
    1,   // "ROUNDBAHTUP",
    1,   // "THAIYEAR",
    255, // "RTD",
    255, // "CUBEVALUE",
    3,   // "CUBEMEMBER",
    3,   // "CUBEMEMBERPROPERTY",
    4,   // "CUBERANKEDMEMBER",
    2,   // "HEX2BIN",
    1,   // "HEX2DEC",
    2,   // "HEX2OCT",
    2,   // "DEC2BIN",
    2,   // "DEC2HEX",
    2,   // "DEC2OCT",
    2,   // "OCT2BIN",
    2,   // "OCT2HEX",
    1,   // "OCT2DEC",
    1,   // "BIN2DEC",
    2,   // "BIN2OCT",
    2,   // "BIN2HEX",
    2,   // "IMSUB",
    2,   // "IMDIV",
    2,   // "IMPOWER",
    1,   // "IMABS",
    1,   // "IMSQRT",
    1,   // "IMLN",
    1,   // "IMLOG2",
    1,   // "IMLOG10",
    1,   // "IMSIN",
    1,   // "IMCOS",
    1,   // "IMEXP",
    1,   // "IMARGUMENT",
    1,   // "IMCONJUGATE",
    1,   // "IMAGINARY",
    1,   // "IMREAL",
    3,   // "COMPLEX",
    255, // "IMSUM",
    255, // "IMPRODUCT",
    4,   // "SERIESSUM",
    1,   // "FACTDOUBLE",
    1,   // "SQRTPI",
    2,   // "QUOTIENT",
    2,   // "DELTA",
    2,   // "GESTEP",
    1,   // "ISEVEN",
    1,   // "ISODD",
    2,   // "MROUND",
    2,   // "ERF",
    1,   // "ERFC",
    2,   // "BESSELJ",
    2,   // "BESSELK",
    2,   // "BESSELY",
    2,   // "BESSELI",
    3,   // "XIRR",
    3,   // "XNPV",
    6,   // "PRICEMAT",
    6,   // "YIELDMAT",
    5,   // "INTRATE",
    5,   // "RECEIVED",
    5,   // "DISC",
    5,   // "PRICEDISC",
    5,   // "YIELDDISC",
    3,   // "TBILLEQ",
    3,   // "TBILLPRICE",
    3,   // "TBILLYIELD",
    7,   // "PRICE",
    7,   // "YIELD",
    2,   // "DOLLARDE",
    2,   // "DOLLARFR",
    2,   // "NOMINAL",
    2,   // "EFFECT",
    6,   // "CUMPRINC",
    6,   // "CUMIPMT",
    2,   // "EDATE",
    2,   // "EOMONTH",
    3,   // "YEARFRAC",
    4,   // "COUPDAYBS",
    4,   // "COUPDAYS",
    4,   // "COUPDAYSNC",
    4,   // "COUPNCD",
    4,   // "COUPNUM",
    4,   // "COUPPCD",
    6,   // "DURATION",
    6,   // "MDURATION",
    8,   // "ODDLPRICE",
    8,   // "ODDLYIELD",
    8,   // "ODDFPRICE",
    8,   // "ODDFYIELD",
    2,   // "RANDBETWEEN",
    2,   // "WEEKNUM",
    7,   // "AMORDEGRC",
    7,   // "AMORLINC",
    8,   // "CONVERT",
    //  1, //    "SHEETJS",
    8,   // "ACCRINT",
    5,   // "ACCRINTM",
    3,   // "WORKDAY",
    3,   // "NETWORKDAYS",
    255, // "GCD",
    255, // "MULTINOMIAL",
    255, // "LCM",
    2,   // "FVSCHEDULE",
    4,   // "CUBEKPIMEMBER",
    5,   // "CUBESET",
    1,   // "CUBESETCOUNT",
    2,   // "IFERROR",
    128, // "COUNTIFS",
    129, // "SUMIFS",
    3,   // "AVERAGEIF",
    129, // "AVERAGEIFS"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_to_u32() {
        let data = b"ABCDEFGH";
        assert_eq!(
            to_u32(data).collect::<Vec<_>>(),
            [u32::from_le_bytes(*b"ABCD"), u32::from_le_bytes(*b"EFGH")]
        );
    }
}
