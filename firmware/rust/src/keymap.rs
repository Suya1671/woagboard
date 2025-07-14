use crate::{COL, ROW};
use rmk::action::KeyAction;
use rmk::config::{CombosConfig, ForksConfig};
use rmk::fork::{Fork, StateBits};
use rmk::heapless::Vec;
use rmk::hid_state::{HidModifiers, HidMouseButtons};
use rmk::keycode::ModifierCombination;
use rmk::light::LedIndicator;
use rmk::{a, k, mt, osm, wm};
pub(crate) const NUM_LAYER: usize = 5;

macro_rules! create_named_macros {
    (($d:tt) $(($name:ident $(, $params:ident:$type:ident)*)),*) => {
        $(
            macro_rules! $name {
                (base $(, $d $params:$type)*) => { rmk::$name!(0 $(, $d $params)*) };
                (nav $(, $d $params:$type)*) => { rmk::$name!(1 $(, $d $params)*) };
                (fn $(, $d $params:$type)*) => { rmk::$name!(2 $(, $d $params)*) };
                (num $(, $d $params:$type)*) => { rmk::$name!(3 $(, $d $params)*) };
                (sys $(, $d $params:$type)*) => { rmk::$name!(4 $(, $d $params)*) };
            }
        )*
    };

    ( $(($name:ident $(, $params:ident:$type:ident)*)),*) => {
        create_named_macros!{($) $(($name $(, $params:$type)*)),*}
    };
}

create_named_macros!((lt, k:ident), (mo));

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL * 2]; ROW]; NUM_LAYER] {
    let shift = ModifierCombination::new_from(false, false, false, true, false);
    let ctrl = ModifierCombination::new_from(false, false, false, false, true);
    let gui = ModifierCombination::new_from(false, true, false, false, false);
    let alt = ModifierCombination::new_from(false, true, true, false, false);

    let right = ModifierCombination::new_from(true, false, false, false, false);
    let rshift = right | shift;
    let rctrl = right | ctrl;
    let rgui = right | gui;
    let ralt = right | alt;

    // 🏳️‍⚧️
    let trans = a!(Transparent);

    // waiting for something like https://github.com/urob/zmk-auto-layer in rmk
    // let smart_num = TapDance::new(mo!(num), hold, hold_after_tap, osl!(num), tapping_term);
    // let smart_shift = TapDance::new(osm!(rshift), k!(RShift), k!(RShift), osl!(num), tapping_term);

    #[rustfmt::skip]
    let base = [
        //  ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮ ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮
            [    k!(X),       k!(F),        k!(D),        k!(P),        k!(Q),           k!(J),      k!(Quote),      k!(O),        k!(U),       k!(Dot)   ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [ mt!(N,ctrl), mt!(S,shift),  mt!(T,alt),   mt!(C,gui),     k!(Y),           k!(M),     mt!(H,rgui),  mt!(A,ralt), mt!(E,rshift), mt!(I,rctrl)],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [    k!(B),       k!(V),        k!(K),        k!(G),        k!(W),           k!(Z),        k!(L),    k!(RepeatKey),  k!(Slash),    k!(Comma)  ],
        //  ╰─────────────┴─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────├─────────────┴─────────────╯
            [a!(No),      a!(No),          k!(LGui),      k!(R),     lt!(fn,Return),    mo!(num),  lt!(nav,Space), k!(RShift),    a!(No),       a!(No)    ]
        //                              ╰─────────────┴─────────────┴─────────────╯ ╰─────────────┴─────────────┴─────────────╯
            // ^ TODO: replace mo!(num) with smart_num and k!(RShift) with smart_shift
    ];

    // TODO: cancelling for caps-word and num-word
    let cancel = trans;

    #[rustfmt::skip]
    let nav = [
        //  ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮ ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮
            [    trans,        trans,     k!(Return),  wm!(Tab, alt),   trans,         k!(PageUp), k!(Backspace),   k!(Up),     k!(Delete),      trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [  osm!(gui),    osm!(alt),   osm!(ctrl),   osm!(shift),    trans,         k!(Left),     k!(Down),      k!(Up),     k!(Right),     k!(Return) ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [    trans,        trans,        trans,       trans,        trans,        k!(PageDown),  k!(Insert),    k!(Tab),      trans,         trans    ],
        //  ╰─────────────┴─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────├─────────────┴─────────────╯
            [trans, trans,                   trans,       trans,        trans,           trans,        trans,       cancel,                   trans, trans]
        //                              ╰─────────────┴─────────────┴─────────────╯ ╰─────────────┴─────────────┴─────────────╯
    ];


    let prev = k!(MediaPrevTrack);
    let play = k!(MediaPlayPause);
    let next = k!(MediaNextTrack);
    let sleep = k!(SystemSleep);

    let desktop_prev = wm!(Left, alt | gui);
    let desktop_next = wm!(Right, alt | gui);

    let volume_up = k!(KbVolumeUp);
    let volume_down = k!(KbVolumeDown);


    #[rustfmt::skip]
    let fn_layer = [
        //  ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮ ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮
            [   k!(F12),       k!(F7),       k!(F8),      k!(F9),        trans,          trans,        prev,         play,         next,         sleep    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [   k!(F11),       k!(F4),       k!(F5),      k!(F6),        trans,          trans,    desktop_prev,   volume_up,  desktop_next,     trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [   k!(F10),       k!(F1),       k!(F2),      k!(F3),        trans,          trans,        trans,     volume_down,      trans,       trans    ],
        //  ╰─────────────┴─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────├─────────────┴─────────────╯
            [trans, trans,                   trans,        trans,        trans,          trans,        trans,     k!(KbMute),                 trans, trans]
        //                              ╰─────────────┴─────────────┴─────────────╯ ╰─────────────┴─────────────┴─────────────╯
    ];

    #[rustfmt::skip]
    let num = [
        //  ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮ ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮
            [    trans,       k!(Kc7),      k!(Kc8),      k!(Kc9),       trans,          trans,        trans,        trans,        trans,        trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [mt!(Kc0, gui),   k!(Kc4),      k!(Kc5),      k!(Kc6),       trans,          trans,        trans,        trans,        trans,        trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [    trans,       k!(Kc1),      k!(Kc2),      k!(Kc3),       trans,          trans,        trans,        trans,        trans,        trans    ],
        //  ╰─────────────┴─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────├─────────────┴─────────────╯
            [trans, trans,                   trans,        trans,    mt!(Kc0, gui),      trans,        trans,        trans,                   trans, trans]
        //                              ╰─────────────┴─────────────┴─────────────╯ ╰─────────────┴─────────────┴─────────────╯
    ];

    #[rustfmt::skip]
    let sys = [
        //  ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮ ╭─────────────┬─────────────┬─────────────┬─────────────┬─────────────╮
            [  k!(User0),    k!(User1),    k!(User2),    k!(User7),      trans,          trans,        trans,        trans,        trans,        trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [    trans,        trans,        trans,        trans,   k!(Bootloader), k!(Bootloader),        trans,        trans,        trans,        trans    ],
        //  ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
            [    trans,        trans,        trans,        trans,        trans,          trans,        trans,        trans,        trans,        trans    ],
        //  ╰─────────────┴─────────────┼─────────────┼─────────────┼─────────────┤ ├─────────────┼─────────────┼─────────────├─────────────┴─────────────╯
            [trans, trans,                   trans,        trans,        trans,          trans,        trans,        trans,                   trans, trans]
        //                              ╰─────────────┴─────────────┴─────────────╯ ╰─────────────┴─────────────┴─────────────╯
    ];

    [
        base,
        nav,
        fn_layer,
        num,
        sys
    ]
}

pub fn get_forks() -> ForksConfig {
    let shift = ModifierCombination::new_from(false, false, false, true, false);

    let match_shift = StateBits::new_from(
        HidModifiers::new_from(false, true, false, false, false, true, false, false),
        LedIndicator::default(),
        HidMouseButtons::default(),
    );

    let match_ctrl = StateBits::new_from(
        HidModifiers::new_from(false, false, true, false, false, false, false, false),
        LedIndicator::default(),
        HidMouseButtons::default(),
    );

    // tap: comma | shift + tap: semicolon | ctrl + shift + tap: <
    let comma_fork = Fork::new(
        k!(Comma),
        k!(Comma),
        k!(Semicolon),
        match_shift,
        StateBits::default(),
        HidModifiers::default(),
        true,
    );
    let comma_inner_fork = Fork::new(
        k!(Semicolon),
        k!(Semicolon),
        wm!(Comma, shift),
        match_ctrl,
        StateBits::default(),
        HidModifiers::default(),
        false,
    );

    // tap: dot | shift + tap: colon | ctrl + shift + tap: >
    let dot_fork = Fork::new(
        k!(Dot),
        k!(Dot),
        wm!(Semicolon, shift),
        match_shift,
        StateBits::default(),
        HidModifiers::default(),
        true,
    );
    let dot_inner_fork = Fork::new(
        wm!(Semicolon, shift),
        wm!(Semicolon, shift),
        wm!(Dot, shift),
        match_ctrl,
        StateBits::default(),
        HidModifiers::default(),
        false,
    );

    ForksConfig {
        forks: Vec::from_slice(&[comma_fork, comma_inner_fork, dot_fork, dot_inner_fork])
            .expect("Failed to create forks"),
    }
}

pub fn get_combos() -> CombosConfig {
    CombosConfig {
        combos: Vec::from_slice(&[]).expect("Failed to create combos"),
        ..Default::default()
    }
}
