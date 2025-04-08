#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

enum CoinAction {
    // 花钱
    castCoin(Coin),

    // 充钱
    chanrgeCion(Coin),
}

struct User {
    cast: u8,
}

impl User{
    fn actionCoin(&mut self ,action:CoinAction){
        match action {
            CoinAction::castCoin(coin) => {
                self.cast -= value_in_cents(coin);
            },

            CoinAction::chanrgeCion(coin) => {
                self.cast += value_in_cents(coin);
            }

        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn main() {
    let coin_value = value_in_cents(Coin::Quarter((UsState::Alabama)));
    print!("当前的值为 {}", coin_value);

    let mut user = User{
        cast:0
    };

    // 进行充值操作
    user.actionCoin(CoinAction::chanrgeCion((Coin::Dime)));

    user.actionCoin(CoinAction::chanrgeCion((Coin::Quarter((UsState::Alaska)))));

    println!("用户当前的余额为 {}", user.cast);

    let coinsEnumArr = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama)
    ];

    let filter_coins_enum_arr:Vec<&Coin> = coinsEnumArr.iter().filter(|coin| matches!(coin, Coin::Penny)).collect();

    print!("{:?}", filter_coins_enum_arr);


}
