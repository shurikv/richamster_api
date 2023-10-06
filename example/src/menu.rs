use enum_iterator::{all, Sequence};

#[derive(Debug, Sequence, PartialEq)]
pub enum MenuItems {
    Login,
    UserDetails,
    UserBalance,
    Quit,
}

impl From<MenuItems> for usize {
    fn from(value: MenuItems) -> Self {
        match value {
            MenuItems::Login => 1,
            MenuItems::UserDetails => 2,
            MenuItems::UserBalance => 3,
            MenuItems::Quit => 4,
        }
    }
}

impl From<usize> for MenuItems {
    fn from(value: usize) -> Self {
        match value {
            1 => MenuItems::Login,
            2 => MenuItems::UserDetails,
            3 => MenuItems::UserBalance,
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
                MenuItems::Quit => "4. Quit",
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
            MenuItems::Quit => "",
        };
        println!("{:^35}", title);
        println!("{:-<35}", "");
    }
}
