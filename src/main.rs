mod ownership;
mod option_and_result;
mod generics;
mod enums;

use crate::ownership::*;
use crate::option_and_result::*;
use crate::generics::*;
use crate::enums::*;

fn main() {
    generics();
    ownership();
    option_and_result();
    enums();
}
