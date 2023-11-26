#[macro_export]
macro_rules! dotenv_get {
    ($var:expr) => {
        dotenv::var($var).expect(concat!("Missing ", $var, " environmental variable"))
    };
    ($var:expr, $parse_as:ty) => {
        dotenv::var($var)
            .expect(concat!("Missing ", $var, " environmental variable"))
            .parse::<$parse_as>()
            .expect(concat!($var, " must be valid ", stringify!($parse_as)))
    };
}

#[macro_export]
macro_rules! create_enums {
    ($($name:ident: [$($count:literal => $variants:ident),*]),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                $(
                    $variants = $count
                ),*
                ,None = 127
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match *self {
                        $(
                            $name::$variants => write!(f, stringify!($variants))
                        ),*
                        ,$name::None => write!(f, "None"),
                    }
                }
            }

            impl std::str::FromStr for $name {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    if let Some(c) = s.chars().next() {
                        return Ok(Self::from(c));
                    }
                    Err(())
                }
            }

            impl From<char> for $name {
                fn from(value: char) -> Self {
                    match value as i8 - '0' as i8 {
                        $(
                            $count => Self::$variants,
                        )*
                        _ => Self::None
                    }
                }
            }

            impl $name {
                #[allow(dead_code)]
                pub fn from_int_unchecked(num: u8) -> Self {
                    unsafe { std::mem::transmute(num) }
                }
            }
        )+
    }
}