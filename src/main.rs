mod ownership;
mod option_and_result;
mod generics;
mod enums;
mod variables;
mod collections;

use crate::ownership::*;
use crate::option_and_result::*;
use crate::generics::*;
use crate::enums::*;
use crate::variables::*;
use crate::collections::*;

fn main() {
    collections();
    variables();
    generics();
    ownership();
    option_and_result();
    enums();
}
