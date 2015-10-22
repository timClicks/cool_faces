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

pub fn angry() -> &'static str {
    let faces = [
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

    choose!(faces)
}

pub fn confused() -> &'static str {
    let faces = [
        "( '-')",
        "⊙﹏⊙",
        "ლ,ᔑ•ﺪ͟͠•ᔐ.ლ",
        "⚆ _ ⚆",
        "ノ( º _ ºノ)",
        "٩◔̯◔۶",
        "ʅʕ•ᴥ•ʔʃ",
    ];

    choose!(faces)
}


pub fn disappointed() -> &'static str {
    let faces = [
        "¬_¬",
        "( ︶︿︶)",
        "(；一_一)",
    ];
    choose!(faces)
}

pub fn excited() -> &'static str {
    let faces = [
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

    choose!(faces)
}

pub fn happy() -> &'static str {
    let faces = [
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

    choose!(faces)
}

pub fn meh() -> &'static str {
    let faces = [
        "¯\\_(ツ)_/¯",
        "( ͡° ͜ʖ ͡°)",
        "(-.-)",
    ];

    choose!(faces)
}

pub fn sad() -> &'static str {
    let faces = [
        "(¡~¡)",
        "( ⚆ _ ⚆ )",
        "༼;´༎ຶ ۝ ༎ຶ༽",
        "༼ ºل͟º ༽",
        "ಠ╭╮ಠ",
        ":(",
    ];

    choose!(faces)
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
    assert_eq!(face, "(╯°□°）╯︵ ┻━┻");
}
