pub const NAME:           &str = "TETRIS";// "TETRIS"

use macroquad::color::*;
pub const BACKGROUND_COLOR:Color = BLACK;
pub const BLOCK_COLOR:     Color = WHITE;

pub const SCORE_INITIAL_VALUE:    i32 = 0;//0
pub const SCORE_ADDITIONAL_VALUE :i32 = 1;//1
pub const HEIGHT:               usize = 20;//20
pub const WIDTH:                usize = 10;//10
pub const PIXELS_IN_BLOCK:       u32 = 20;//20
pub const DELAY:                  u64 = 1000; //ms(1000)

pub const GAME_OVER_DELAY:    u64 = 1000; //ms(1000)


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