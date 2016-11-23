pub enum Step<M, O, C> {
    NotReady(M, O),
    Done(C),
}

/// Shorthand:
type AResult<M: MealyMachine> = Result<Step<M, Option<M::Output>, M::CalcResult>, M::Error>;

pub trait MealyMachine: Sized {
    type Input;
    type Output;
    type Error;
    type CalcResult;

    fn transition(self,
                  Self::Input)
                  -> Result<Step<Self, Option<Self::Output>, Self::CalcResult>, Self::Error>;

    fn and_then<M, F>(self, m: M, f: F) -> AndThen<Self, M, F>
        where M: MealyMachine<Input = Self::Input, Output = Self::Output, Error = Self::Error>,
              F: FnOnce(Self::CalcResult) -> M
    {
        AndThen::Machine1(self, f)
    }
}

pub enum AndThen<M1, M2, F>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>,
          F: FnOnce(M1::CalcResult) -> M2
{
    Machine1(M1, F),
    Machine2(M2),
}


impl<M1, M2, F> MealyMachine for AndThen<M1, M2, F>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>,
          F: FnOnce(M1::CalcResult) -> M2
{
    type Input = M1::Input;
    type Output = M1::Output;
    type Error = M1::Error;
    type CalcResult = M2::CalcResult;

    fn transition(self,
                  input: Self::Input)
                  -> Result<Step<Self, Option<Self::Output>, Self::CalcResult>, Self::Error> {
        match self {
            AndThen::Machine1(m1, f) => {
                match m1.transition(input)? {
                    Step::NotReady(new_m1, output) => {
                        Ok(Step::NotReady(AndThen::Machine1(new_m1, f), output))
                    }
                    Step::Done(cresult) => Ok(Step::NotReady(AndThen::Machine2(f(cresult)), None)),
                }
            }
            AndThen::Machine2(m2) => {
                match m2.transition(input)? {
                    Step::NotReady(new_m2, output) => {
                        Ok(Step::NotReady(AndThen::Machine2(new_m2), output))
                    }
                    Step::Done(cresult) => Ok(Step::Done(cresult)),
                }
            }
        }
    }
}
