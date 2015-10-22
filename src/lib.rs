extern crate rand;

use rand::random;

macro_rules! choose {
    ( $x:expr ) => {
        {
            let length = $x.len() as f32;
            let idx = rand::random::<f32>() * length;
            $x[idx.floor() as usize]
        }
    };
}

const ANGRY_FACES: &'static [&'static str] = &[
    "(╯°□°）╯︵ ┻━┻",
    "┻━┻︵ \\(°□°)/ ︵ ┻━┻",
    "(ಠ_ಠ)",
    "(⌐■_■)",
    "ヾ(⌐■_■)ノ",
    "ヽ(`Д´)ﾉ",
    "ರ_ರ",
    "(ノಠ益ಠ)ノ",
    "ノಠ益ಠ)ノ彡",
    "( ͠° ͟ʖ ͡°)﻿",
    "ᕦ(ò_óˇ)ᕤ",
    "（╯°□°）╯︵(\\ .o.)\\",
    "(ง ͠° ͟ل͜ ͡°)ง",
    "(ง ͡ʘ ͜ʖ ͡ʘ)ง",
    "(ง •̀_•́)ง",
    "┌( ಠ_ಠ)┘",
    "╚(ಠ_ಠ)=┐",
    "(۶ૈ ۜ ᵒ̌▱๋ᵒ̌ )۶ૈ=͟͟͞͞ ⌨",
    "꒰✘Д✘◍꒱",
    "( `·´ )",
];

pub fn angry() -> &'static str {
    choose!(ANGRY_FACES)
}

const CONFUSED_FACES: &'static [&'static str] = &[
    "( '-')",
    "⊙﹏⊙",
    "ლ,ᔑ•ﺪ͟͠•ᔐ.ლ",
    "⚆ _ ⚆",
    "ノ( º _ ºノ)",
    "٩◔̯◔۶",
    "ʅʕ•ᴥ•ʔʃ",

];
pub fn confused() -> &'static str {
    choose!(CONFUSED_FACES)
}

const DISAPPOINTED_FACES: &'static [&'static str] = &[
    "¬_¬",
    "( ︶︿︶)",
    "(；一_一)",
];

pub fn disappointed() -> &'static str {
    choose!(DISAPPOINTED_FACES)
}

const EXCITED_FACES: &'static [&'static str] = &[
    "☜(⌒▽⌒)☞",
    "ヽ༼ຈل͜ຈ༽ﾉ",
    "ᕕ( ᐛ )ᕗ",
    "ᕙ༼ຈل͜ຈ༽ᕗ",
    "ᕙ༼ ,,ԾܫԾ,, ༽ᕗ",
    "\\m/_(>_<)_\\m/",
    "/(^.^/)",
    "(ﾉ◕ヮ◕)ﾉ",
    "t(-.-t)",
    "ヽ༼ʘ̚ل͜ʘ̚༽ﾉ",
    "ヽ༼ຈل͜ຈ༽ง",
    "ヽ༼ຈل͜ຈ༽ﾉ",
    "ヽ༼Ὸل͜ຈ༽ﾉ",
    "ヾ(⌐■_■)ノ",
];

pub fn excited() -> &'static str {
    choose!(EXCITED_FACES)
}

const HAPPY_FACES: &'static [&'static str] = &[
    "( ͜。 ͡ʖ ͜。)",
    "~(‾▿‾)~",
    "( ͡° ͜ʖ ͡°)",
    "(\\_/)",
    "╚(▲_▲)╝",
    "(‾⌣‾)♉",
    "(˚◡˚)",
    "( ﾟヮﾟ)",
    "٩(❛ᴗ❛)۶",
    "(｡◕ ‿ ◕｡)",
    "(ʘ‿ʘ)",
    "(ಠ‿ಠ)",
    "(ಠ⌣ಠ)",
    "(ღ˘⌣˘ღ)",
    "(ღ˘⌣˘ღ)",
    "(ᵔᴥᵔ)",
    "(•ω•)",
    "(•◡•)/",
    "=^.^=",
    "☼.☼",
    "♥‿♥",
    "ʘ‿ʘ",
    "° ͜ʖ ͡°",
];

pub fn happy() -> &'static str {
    choose!(HAPPY_FACES)
}

const MEH_FACES: &'static [&'static str] = &[
    "¯\\_(ツ)_/¯",
    "( ͡° ͜ʖ ͡°)",
    "(-.-)",
];

pub fn meh() -> &'static str {
    choose!(MEH_FACES)
}

const SAD_FACES: &'static [&'static str] = &[
    "(¡~¡)",
    "( ⚆ _ ⚆ )",
    "༼;´༎ຶ ۝ ༎ຶ༽",
    "༼ ºل͟º ༽",
    "ಠ╭╮ಠ",
    ":(",
];

pub fn sad() -> &'static str {
    choose!(SAD_FACES)
}

pub fn deal_with_it() -> &'static str {
    "(⌐■_■)"
}

// TODO: figure out how to build a
// collection of functions

// pub fn face() -> String {
//     let fns = vec![
//         happy,
//         sad
//     ];
//
//     choose!(fns)()
// }

#[test]
fn test_angry() {
    let face = angry();
    println!("{}", face);
    assert!(ANGRY_FACES.contains(&face));
}
