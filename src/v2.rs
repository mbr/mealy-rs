
// transition(self, Self::Input) -> Result<Step, AError>


enum Step<O, C> {
    NotReady(O),
    Done(O, C),
}


trait NewAutomaton: Sized {
    type Input;
    type Output;
    type Error;
    type CalcResult;

    fn transition(self,
                  Self::Input)
                  -> Result<(Self, Step<Self::Output, Self::CalcResult>), Self::Error>;

    fn and_then<M: NewAutomaton<Input = Self::Input, Output = Self::Output, Error = Self::Error>>
        (self,
         m: M)
         -> AndThen<Self, M> {
        AndThen::Machine1(self, m)
    }
}

enum AndThen<M1, M2>
    where M1: NewAutomaton,
          M2: NewAutomaton<Input = M1::Input, Output = M1::Output, Error = M1::Error>
{
    Machine1(M1, M2),
    Machine2(M2),
}


impl<M1, M2> NewAutomaton for AndThen<M1, M2>
    where M1: NewAutomaton,
          M2: NewAutomaton<Input = M1::Input, Output = M1::Output, Error = M1::Error>
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
                let (new_m1, output) = m1.transition(input)?;
                match output {
                    Step::NotReady(output) => {
                        Ok((AndThen::Machine1(new_m1, m2), Step::NotReady(output)))
                    }
                    Step::Done(output, cresult) => {
                        Ok((AndThen::Machine2(m2), Step::NotReady(output)))
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}
