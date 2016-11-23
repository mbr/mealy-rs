//! Docs pending. Sorry.

#![no_std]

pub mod timeout;
pub mod v2;

pub trait MealyAutomaton: Sized {
    type Input;
    type Output;

    fn transition(self, input: Self::Input) -> (Self, Self::Output);

    #[inline]
    fn failed(&self) -> bool {
        false
    }

    #[inline]
    fn done(&self) -> bool {
        false
    }

    #[inline]
    fn halted(&self) -> bool {
        self.failed() || self.done()
    }
}
