use ::MealyAutomaton;

pub struct Timeout<M, T> {
    inner: M,
    timed_out: bool,
    limit: T,
}

pub enum TimeoutInput<I, T> {
    Input(I),
    UpdateTime(T),
}

impl<M, T, I, O> Timeout<M, T>
    where M: MealyAutomaton<Input = I, Output = O>
{
    pub fn new(m: M, times_out_at: T) -> Timeout<M, T> {
        Timeout {
            inner: m,
            timed_out: false,
            limit: times_out_at,
        }
    }
}

impl<M, T, I, O> MealyAutomaton for Timeout<M, T>
    where M: MealyAutomaton<Input = I, Output = O>,
          T: PartialOrd
{
    type Input = TimeoutInput<I, T>;
    type Output = Option<O>;

    fn transition(mut self, input: Self::Input) -> (Self, Self::Output) {
        if self.halted() {
            return (self, None);
        }

        match input {
            TimeoutInput::Input(inner_input) => {
                let output;
                self.inner = {
                    let (new_state, ioutput) = self.inner.transition(inner_input);
                    output = ioutput;
                    new_state
                };

                (self, Some(output))
            }

            TimeoutInput::UpdateTime(now) => {
                if now >= self.limit {
                    self.timed_out = true;
                    (self, None)
                } else {
                    (self, None)
                }
            }
        }
    }

    fn failed(&self) -> bool {
        self.timed_out || self.inner.failed()
    }

    fn done(&self) -> bool {
        self.inner.done()
    }
}
