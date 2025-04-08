/*
 * @Author: xh20220630 haox84961@gmail.com
 * @Date: 2025-01-12 12:30:41
 * @LastEditors: xh20220630 haox84961@gmail.com
 * @LastEditTime: 2025-01-12 13:04:13
 * @FilePath: \rust-project\src\composite_type\my_enum\mod.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

static CLUBS: Vec<PokerCard> = Vec::new();
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

// struct PokerCard {
//     suit: PokerSuit,
//     value: u8,
// }
#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

// fn build_poker_card(suit: PokerSuit, value: u8) -> PokerCard {
//     PokerCard {
//         suit: suit,
//         value: value,
//     }
// }

pub fn main() {
    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;

    // let c1 = build_poker_card(PokerSuit::Clubs, 1);
    // let c2 = build_poker_card(PokerSuit::Spades, 2);
    // let c3 = build_poker_card(PokerSuit::Diamonds, 3);
    // let c4 = build_poker_card(PokerSuit::Hearts, 4);

    let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);

   print!("{:?}", c1);
   println!("{:?}", c2);

}
