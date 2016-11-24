extern crate mealy;

use mealy::v2::*;

struct FooMachine(bool);

impl MealyMachine for FooMachine {
    type Input = ();
    type Output = ();
    type Error = &'static str;
    type CalcResult = ();
    fn transition(self, (): ()) -> AResult<Self> {
        if self.0 {
            Ok(Step::NotReady(FooMachine(false), ()))
        } else {
            Ok(Step::Done(()))
        }
    }
}

struct Foo {
    foo_state: FooMachine,
    _other: String,
}

impl MealyMachine for Foo {
    type Input = ();
    type Output = ();
    type Error = &'static str;
    type CalcResult = ();
    fn transition(mut self, (): ()) -> AResult<Self> {
        self.foo_state = match self.foo_state.transition(())? {
            Step::Done(()) => return Ok(Step::Done(())),
            Step::NotReady(next, ()) => next,
        };
        Ok(Step::NotReady(self, ()))
    }
}

#[test]
#[should_panic = "badaa"]
fn initial_example() {
    let foo = Foo {
        foo_state: FooMachine(true),
        _other: "foo".to_string(),
    };
    let (foo, ()) = foo.transition(()).unwrap()
                       .try_next_state().unwrap();
    let (foo, ()) = foo.transition(()).unwrap()
                       .try_next_state().expect("badaa");
    let (_, ()) = foo.transition(()).unwrap()
                       .try_next_state().unwrap();
}
