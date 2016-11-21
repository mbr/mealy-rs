#![no_std]

pub trait MealyAutomaton: Sized {
    type Input;
    type Output;

    fn transition(self, input: Self::Input) -> (Self, Self::Output);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
