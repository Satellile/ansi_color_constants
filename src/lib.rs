pub const RESET: &str = "\x1b[0m";

pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const BLINK: &str = "\x1b[5m"; // on Windows Terminal, only works when terminal has window focus. annoying bc im using it on a side monitor, and specifically want to use blinking text to capture my attention for important things
pub const UNDIM: &str = "\x1b[22m";
pub const ITALICS: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const BACKGROUND: &str = "\x1b[7m";
pub const STRIKETHRU: &str = "\x1b[9m";
pub const UNDERLINE2: &str = "\x1b[21m";
pub const UNDERLINE3: &str = "\x1b[52m";


pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m"; // name set in stone. usually use for errors
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m"; // name set in stone. usually use for warnings
pub const D_BLUE: &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";
pub const L_BLUE: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const GREY: &str = "\x1b[90m";
pub const L_RED: &str = "\x1b[91m";
pub const L_GREEN: &str = "\x1b[92m";
pub const L_YELLOW: &str = "\x1b[93m";
pub const BLUE: &str = "\x1b[94m";
pub const L_PURPLE: &str = "\x1b[95m";
pub const CYAN: &str = "\x1b[96m"; // name set in stone. usually use for lots of things because i like it :D
pub const BRIGHT: &str = "\x1b[97m";


pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_D_BLUE: &str = "\x1b[44m";
pub const BG_PURPLE: &str = "\x1b[45m";
pub const BG_L_BLUE: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

pub const BG_GREY: &str = "\x1b[100m";
pub const BG_L_RED: &str = "\x1b[101m";
pub const BG_L_GREEN: &str = "\x1b[102m";
pub const BG_L_YELLOW: &str = "\x1b[103m";
pub const BG_BLUE: &str = "\x1b[104m";
pub const BG_L_PURPLE: &str = "\x1b[105m";
pub const BG_CYAN: &str = "\x1b[106m";
pub const BG_BRIGHT: &str = "\x1b[107m";