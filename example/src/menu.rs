use enum_iterator::{Sequence, all};

#[derive(Debug, Sequence, PartialEq)]
#[repr(u8)]

pub enum MenuItems {
    Login = 1,

    UserDetails = 2,

    UserBalance = 3,

    UserTransactions = 4,
    TickerList = 5,
    RestrictionsList = 6,

    Quit = 7,
}

impl From<usize> for MenuItems {
    fn from(value: usize) -> Self {
        match value {
            1 => MenuItems::Login,

            2 => MenuItems::UserDetails,

            3 => MenuItems::UserBalance,

            4 => MenuItems::UserTransactions,

            5 => MenuItems::TickerList,

            6 => MenuItems::RestrictionsList,

            _ => MenuItems::Quit,
        }
    }
}

pub struct Menu;

impl Menu {
    pub fn print() {
        println!("{:-<35}", "");

        println!("{:^35}", "Menu");

        println!("{:-<35}", "");

        for item in all::<MenuItems>().collect::<Vec<_>>() {
            let output_string = match item {
                MenuItems::Login => "1. Login",

                MenuItems::UserDetails => "2. User detail",

                MenuItems::UserBalance => "3. User balance",

                MenuItems::UserTransactions => "4. User transactions",

                MenuItems::TickerList => "5. Ticker list",

                MenuItems::RestrictionsList => "6. Restrictions list",

                MenuItems::Quit => "7. Quit",
            };

            println!("{:?}", output_string);
        }

        println!("{:-<35}", "");
    }

    pub fn print_header(item: &MenuItems) {
        println!("{:-<35}", "");

        let title = match item {
            MenuItems::Login => "Login",

            MenuItems::UserDetails => "User detail",

            MenuItems::UserBalance => "User balance",

            MenuItems::UserTransactions => "User transactions",

            MenuItems::TickerList => "Ticker list",

            MenuItems::RestrictionsList => "Restrictions list",

            MenuItems::Quit => "",
        };

        println!("{:^35}", title);

        println!("{:-<35}", "");
    }
}
