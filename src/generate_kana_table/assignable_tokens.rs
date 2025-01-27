use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Assignable {
    pub Token: String,
    pub Sequence: String,
}

#[derive(EnumString, Display, Serialize, Deserialize, Debug)]
pub enum AssignableTokens {
    Q,
    L,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Period,
    #[strum(serialize = "-")]
    Dash,
    J,
    X,
    C,
    V,
    #[strum(serialize = ";")]
    Semicoron,
    F,
    W,
    R,
    Y,
    P,
    #[strum(serialize = "\\")]
    Backslash,
    #[strum(serialize = "'")]
    Quotation,
    H,
    Ks,
    N,
    T,
    K,
    G,
    D,
    M,
    Z,
    B,
    #[strum(serialize = "/")]
    Slash,
}
