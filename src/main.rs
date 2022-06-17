mod ownership;
mod option_and_result;
mod generics;

use crate::ownership::*;
use crate::option_and_result::*;
use crate::generics::*;

fn main() {
    generics();
    ownership();
    option_and_result();
}
