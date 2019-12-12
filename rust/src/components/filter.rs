use std::fmt;

#[derive(Copy, Clone)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::All => "all",
            Self::Active => "active",
            Self::Completed => "completed"
        };
        write!(f, "{}", name)
    }
}