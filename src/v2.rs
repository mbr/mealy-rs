pub enum Step<O, C> {
    NotReady(O),
    Done(O, C),
}


pub trait MealyMachine: Sized {
    type Input;
    type Output;
    type Error;
    type CalcResult;

    fn transition(self,
                  Self::Input)
                  -> Result<(Self, Step<Self::Output, Self::CalcResult>), Self::Error>;

    fn and_then<M: MealyMachine<Input = Self::Input, Output = Self::Output, Error = Self::Error>>
        (self,
         m: M)
         -> AndThen<Self, M> {
        AndThen::Machine1(self, m)
    }
}

pub enum AndThen<M1, M2>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>
{
    Machine1(M1, M2),
    Machine2(M2),
    Done,
}


impl<M1, M2> MealyMachine for AndThen<M1, M2>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>
{
    type Input = M1::Input;
    type Output = M1::Output;
    type Error = M1::Error;
    type CalcResult = M2::CalcResult;

    fn transition(self,
                  input: Self::Input)
                  -> Result<(Self, Step<Self::Output, Self::CalcResult>), Self::Error> {
        match self {
            AndThen::Machine1(m1, m2) => {
                let (new_m1, step) = m1.transition(input)?;
                match step {
                    Step::NotReady(output) => {
                        Ok((AndThen::Machine1(new_m1, m2), Step::NotReady(output)))
                    }
                    Step::Done(output, cresult) => {
                        Ok((AndThen::Machine2(m2), Step::NotReady(output)))
                    }
                }
            }
            AndThen::Machine2(m2) => {
                let (new_m2, step) = m2.transition(input)?;

                match step {
                    Step::NotReady(output) => {
                        Ok((AndThen::Machine2(new_m2), Step::NotReady(output)))
                    }
                    Step::Done(output, cresult) => Ok((AndThen::Done, Step::Done(output, cresult))),
                }
            }
            AndThen::Done => unimplemented!(),
        }
    }
}
