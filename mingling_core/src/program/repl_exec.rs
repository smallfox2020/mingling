use crate::{Program, ProgramCollect};

#[cfg(not(feature = "async"))]
impl<C> Program<C>
where
    C: ProgramCollect<Enum = C>,
{
    pub fn exec_repl(self) {}
}

#[cfg(feature = "async")]
impl<C> Program<C> where C: ProgramCollect<Enum = C> {}
