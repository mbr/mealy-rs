pub enum Step<M, O, C> {
    NotReady(M, O),
    Done(M, O, C),
}

/// Shorthand:
type AResult<M: MealyMachine> = Result<Step<M, M::Output, M::CalcResult>, M::Error>;

pub trait MealyMachine: Sized {
    type Input;
    type Output;
    type Error;
    type CalcResult;

    fn transition(self, Self::Input) -> Result<Step<Self, Self::Output, Self::CalcResult>, Self::Error>;

    fn and_then<M>(self, m: M) -> AndThen<Self, M>
        where M : MealyMachine<Input = Self::Input, Output = Self::Output, Error = Self::Error> {
        AndThen::Machine1(self, m)
    }
}

pub enum AndThen<M1, M2>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>
{
    Machine1(M1, M2),
    Machine2(M2),
}


impl<M1, M2> MealyMachine for AndThen<M1, M2>
    where M1: MealyMachine,
          M2: MealyMachine<Input = M1::Input, Output = M1::Output, Error = M1::Error>
{
    type Input = M1::Input;
    type Output = M1::Output;
    type Error = M1::Error;
    type CalcResult = M2::CalcResult;

    fn transition(self, input: Self::Input) -> Result<Step<Self, Self::Output, Self::CalcResult>, Self::Error> {
        match self {
            AndThen::Machine1(m1, m2) => {
                match m1.transition(input)? {
                    Step::NotReady(new_m1, output) => {
                        Ok(Step::NotReady(AndThen::Machine1(new_m1, m2), output))
                    }
                    Step::Done(new_m1, output, cresult) => {
                        Ok(Step::NotReady(AndThen::Machine2(m2), output))
                    }
                }
            }
            _ => unimplemented!()
//             AndThen::Machine2(m2) => {
//                 let (new_m2, step) = m2.transition(input)?;

//                 match step {
//                     Step::NotReady(output) => {
//                         Ok((AndThen::Machine2(new_m2), Step::NotReady(output)))
//                     }
//                     Step::Done(output, cresult) => Ok((AndThen::Done, Step::Done(output, cresult))),
//                 }
//             }
//             AndThen::Done => unimplemented!(),
        }
    }
}
