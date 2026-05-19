#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::Write;

mod splitter;

use crate::error::{ProgramInternalExecuteError, ProgramPanic};
use crate::program::repl_exec::splitter::split_input_string;
use crate::{Program, ProgramCollect, RenderResult};

#[cfg(not(feature = "async"))]
impl<C> Program<C>
where
    C: ProgramCollect<Enum = C> + Send + Sync + 'static,
{
    /// Executes the REPL interactive CLI mode.
    ///
    /// This method starts an infinite loop that continuously reads user input, parses commands, executes them,
    /// and displays the execution result or error message. It is suitable for scenarios requiring command-line interaction with the user.
    pub fn exec_repl(self) {
        self.run_hook_repl_on_begin();

        self.exec_wrapper(|p| -> ! {
            loop {
                p.run_hook_repl_pre_readline();
                let readline = readline_or_empty();
                p.run_hook_repl_post_readline(&readline);

                let args = split_input_string(readline.clone());

                match exec_once(p, args) {
                    Ok(r) => {
                        p.run_hook_repl_on_receive_result(&r);
                    }
                    Err(ProgramInternalExecuteError::REPLPanic(panic)) => {
                        p.run_hook_repl_on_panic(&panic);
                    }
                    _ => {}
                }
            }
        });
    }
}

#[cfg(feature = "async")]
impl<C> Program<C>
where
    C: ProgramCollect<Enum = C> + Send + Sync + 'static,
{
    /// Executes the REPL interactive CLI mode.
    ///
    /// This method starts an infinite loop that continuously reads user input, parses commands, executes them,
    /// and displays the execution result or error message. It is suitable for scenarios requiring command-line interaction with the user.
    ///
    /// **Note:** When the `async` feature is enabled, panic unwinding is not supported.
    /// Any panics during command execution will result in an abort rather than being caught and handled gracefully.
    pub async fn exec_repl(self) {
        self.run_hook_repl_on_begin();

        self.exec_wrapper(async |p| -> ! {
            loop {
                p.run_hook_repl_pre_readline();
                let readline = readline_or_empty();
                p.run_hook_repl_post_readline(&readline);

                let args = split_input_string(readline.clone());

                match exec_once(p, args).await {
                    Ok(r) => {
                        p.run_hook_repl_on_receive_result(&r);
                    }
                    _ => {}
                }
            }
        })
        .await;
    }
}

fn readline() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn readline_or_empty() -> String {
    readline().unwrap_or("".to_string())
}

#[cfg(not(feature = "async"))]
fn exec_once<C>(
    p: &'static Program<C>,
    args: Vec<String>,
) -> Result<RenderResult, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = C> + Send + Sync + 'static,
{
    #[cfg(panic = "abort")]
    let exec_result = super::exec::exec_with_args(p, args);

    #[cfg(not(panic = "abort"))]
    let exec_result = {
        let exec_unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            super::exec::exec_with_args(p, args)
        }));

        match exec_unwind_result {
            Err(panic_info) => {
                let panic_payload = ProgramPanic {
                    payload: panic_info,
                };
                let program = crate::program::THIS_PROGRAM
                    .get()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<Program<C>>()
                    .unwrap();
                program.run_hook_repl_on_panic(&panic_payload);
                Err(ProgramInternalExecuteError::REPLPanic(panic_payload))
            }
            Ok(r) => r,
        }
    };

    exec_result
}

#[cfg(feature = "async")]
async fn exec_once<C>(
    p: &'static Program<C>,
    args: Vec<String>,
) -> Result<RenderResult, ProgramInternalExecuteError>
where
    C: ProgramCollect<Enum = C> + Send + Sync + 'static,
{
    super::exec::exec_with_args(p, args).await
}
