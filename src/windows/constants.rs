use input::Input;
use pdcurses::chtype;

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_BLUE: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_RED: i16 = 4;
pub const COLOR_CYAN: i16 = (COLOR_BLUE | COLOR_GREEN);
pub const COLOR_MAGENTA: i16 = (COLOR_RED | COLOR_BLUE);
pub const COLOR_YELLOW: i16 = (COLOR_RED | COLOR_GREEN);
pub const COLOR_WHITE: i16 = 7;

pub const PDC_CHARTEXT_BITS: chtype = 21;
pub const PDC_COLOR_SHIFT: chtype = PDC_CHARTEXT_BITS + 12;

pub const A_NORMAL: chtype = 0;
pub const A_COLOR: chtype = 0x7fffffff << PDC_COLOR_SHIFT;
pub const A_BOLD: chtype = 0x080 << PDC_CHARTEXT_BITS;
pub const A_BLINK: chtype = 0x040 << PDC_CHARTEXT_BITS;

pub const KEY_OFFSET: i32 = 0xec00;
pub const KEY_F15: i32 = (KEY_OFFSET + 0x17);
pub const KEY_UNDO: i32 = (KEY_OFFSET + 0x96);
pub const KEY_RESIZE: i32 = (KEY_OFFSET + 0x122);

pub const SPECIAL_KEY_CODES: [Input; 102] = [Input::KeyCodeYes,

                                             Input::KeyBreak,
                                             Input::KeyDown,
                                             Input::KeyUp,
                                             Input::KeyLeft,
                                             Input::KeyRight,
                                             Input::KeyHome,
                                             Input::KeyBackspace,

                                             Input::KeyF0,
                                             Input::KeyF1,
                                             Input::KeyF2,
                                             Input::KeyF3,
                                             Input::KeyF4,
                                             Input::KeyF5,
                                             Input::KeyF6,
                                             Input::KeyF7,
                                             Input::KeyF8,
                                             Input::KeyF9,
                                             Input::KeyF10,
                                             Input::KeyF11,
                                             Input::KeyF12,
                                             Input::KeyF13,
                                             Input::KeyF14,
                                             Input::KeyF15,
                                             // PDcurses reserves space for 64 function keys, but we've
                                             // only implemented 15. This has to be taken into account
                                             // when converting the integer into an index of this array
                                             Input::KeyDL,
                                             Input::KeyIL,
                                             Input::KeyDC,
                                             Input::KeyIC,
                                             Input::KeyEIC,
                                             Input::KeyClear,
                                             Input::KeyEOS,
                                             Input::KeyEOL,
                                             Input::KeySF,
                                             Input::KeySR,
                                             Input::KeyNPage,
                                             Input::KeyPPage,
                                             Input::KeySTab,
                                             Input::KeyCTab,
                                             Input::KeyCATab,
                                             Input::KeyEnter,
                                             Input::KeySReset,
                                             Input::KeyReset,
                                             Input::KeyPrint,
                                             Input::KeyLL,
                                             Input::KeyAbort,
                                             Input::KeySHelp,
                                             Input::KeyLHelp,
                                             Input::KeyBTab,
                                             Input::KeyBeg,
                                             Input::KeyCancel,
                                             Input::KeyClose,
                                             Input::KeyCommand,
                                             Input::KeyCopy,
                                             Input::KeyCreate,
                                             Input::KeyEnd,
                                             Input::KeyExit,
                                             Input::KeyFind,
                                             Input::KeyHelp,
                                             Input::KeyMark,
                                             Input::KeyMessage,
                                             Input::KeyMove,
                                             Input::KeyNext,
                                             Input::KeyOpen,
                                             Input::KeyOptions,
                                             Input::KeyPrevious,
                                             Input::KeyRedo,
                                             Input::KeyReference,
                                             Input::KeyRefresh,
                                             Input::KeyReplace,
                                             Input::KeyRestart,
                                             Input::KeyResume,
                                             Input::KeySave,
                                             Input::KeySBeg,
                                             Input::KeySCancel,
                                             Input::KeySCommand,
                                             Input::KeySCopy,
                                             Input::KeySCreate,
                                             Input::KeySDC,
                                             Input::KeySDL,
                                             Input::KeySelect,
                                             Input::KeySEnd,
                                             Input::KeySEOL,
                                             Input::KeySExit,
                                             Input::KeySFind,
                                             Input::KeySHome,
                                             Input::KeySIC,

                                             Input::KeySLeft,
                                             Input::KeySMessage,
                                             Input::KeySMove,
                                             Input::KeySNext,
                                             Input::KeySOptions,
                                             Input::KeySPrevious,
                                             Input::KeySPrint,
                                             Input::KeySRedo,
                                             Input::KeySReplace,
                                             Input::KeySRight,
                                             Input::KeySResume,
                                             Input::KeySSave,
                                             Input::KeySSuspend,
                                             Input::KeySUndo,
                                             Input::KeySuspend,
                                             Input::KeyUndo];
