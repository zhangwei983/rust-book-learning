// Declarative macros
#[macro_export]
macro_rules! my_vec {
    ( $($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! add_as {
    ( $($a:expr), *) => {
        {
            0 $(+$a)*
        }
    };
}

#[macro_export]
macro_rules! caculate {
    ( $x:expr ) => {
        {
            let value: i32 = $x;
            println!("{} = {}", stringify!{$x}, value);
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}
