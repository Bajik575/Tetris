pub const NAME:           &str = "TETRIS";// "TETRIS"
pub const NAME_LEN:      usize = 6;// 6
pub const NAME_DECORATOR: &str = "~";// "-"

pub const SCORE_INITIAL_VALUE:   i32 = 0;//0
pub const SCORE_ADDITIONAL_VALUE:i32 = 1;//1
pub const HEIGHT: usize = 20;//20
pub const WIDTH:  usize = 10;//10
pub const DELAY:    u64 = 1000; //ms(1000)


pub const SYMBOL_0: char = ' ';//' '
pub const SYMBOL_1: char = '@';//'@'

pub const HORIZONTAL_SYMBOL: char = '═';//'_'
pub const VERTICAL_SYMBOL:   char = '║';//'|'

pub const LEFT_UP_SYMBOL:    char = '╔';//'╔'
pub const RIGHT_UP_SYMBOL:   char = '╗';//'╗'
pub const LEFT_DOWN_SYMBOL:  char = '╚';//'╚'
pub const RIGHT_DOWN_SYMBOL: char = '╝';//'╝'


pub const GAME_OVER_DELAY:    u64 = 1000; //ms(1000)
pub const GAME_OVER_TEXT: &str = "\
\n\n\n\n\n\n
╔═══╗         ╔═══╗
║╔═╗║         ║╔═╗║
║║ ╚╬══╦══╦══╗║║ ║╠╗╔╦══╦═╗
║║═╗║╔╗║║║║ ═╣║║ ║║╚╝║ ═╣╔╝
║╚═╝║╔╗║║║║ ═╣║╚═╝╠╗╔╣ ═╣║
╚═══╩╝╚╩╩╩╩══╝╚═══╝╚╝╚══╩╝
\n\n\n\n\n\n\n";


// Системные константы( НЕ ИЗМЕНЯТЬ )
use crate::Point;
pub const POINTS_IN_FIGURE: usize = 4;
pub const AMOUNT_FIGURES_TYPES: usize = 5;
const FIGURES_TYPES1: [[Point; POINTS_IN_FIGURE]; AMOUNT_FIGURES_TYPES] = [
    [
        Point(0, 0),
        Point(1, 0), // @@@@
        Point(2, 0),
        Point(3, 0),
    ],
    [
        Point(0, 0),
        Point(1, 0), // @@@
        Point(2, 0), //   @
        Point(2, 1),
    ],
    [
        Point(0, 0),
        Point(1, 0), // @@
        Point(0, 1), // @@
        Point(1, 1),
    ],
    [
        Point(0, 0),
        Point(1, 0), // @@@
        Point(2, 0), //  @
        Point(1, 1),
    ],
    [
        Point(1, 0),
        Point(2, 0), //  @@
        Point(0, 1), // @@
        Point(1, 1),
    ],
];
const FIGURES_TYPES2: [[Point; POINTS_IN_FIGURE]; AMOUNT_FIGURES_TYPES] = [
    [
        Point(0, 0), // @
        Point(0, 1), // @
        Point(0, 2), // @
        Point(0, 3), // @
    ],
    [
        Point(1, 0),
        Point(1, 1), //  @
        Point(0, 2), //  @
        Point(1, 2), // @@
    ],
    FIGURES_TYPES1[2],
    [
        Point(1, 0),
        Point(0, 1), //  @
        Point(1, 1), // @@
        Point(1, 2), //  @
    ],
    [
        Point(0, 0),
        Point(0, 1), // @
        Point(1, 1), // @@
        Point(1, 2), //  @
    ],
];
const FIGURES_TYPES3: [[Point; POINTS_IN_FIGURE]; AMOUNT_FIGURES_TYPES] = [
    FIGURES_TYPES1[0],
    [
        Point(0, 0),
        Point(0, 1), // @
        Point(1, 1), // @@@
        Point(2, 1),
    ],
    FIGURES_TYPES1[2],
    [
        Point(1, 0),
        Point(0, 1), //  @
        Point(1, 1), // @@@
        Point(2, 1), //
    ],
    FIGURES_TYPES2[4],
];

const FIGURES_TYPES4: [[Point; POINTS_IN_FIGURE]; AMOUNT_FIGURES_TYPES] = [
    FIGURES_TYPES2[0],
    [
        Point(0, 0),
        Point(1, 0), // @@
        Point(0, 1), // @
        Point(0, 2), // @
    ],
    FIGURES_TYPES1[2],
    [
        Point(0, 0),
        Point(0, 1), // @
        Point(1, 1), // @@
        Point(0, 2), // @
    ],
    FIGURES_TYPES1[4],
];
pub const FIGURES_TYPES: [[[Point; POINTS_IN_FIGURE]; AMOUNT_FIGURES_TYPES]; 4] = [
    FIGURES_TYPES1,
    FIGURES_TYPES2,
    FIGURES_TYPES3,
    FIGURES_TYPES4,
];