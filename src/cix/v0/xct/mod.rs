use serde::{Serialize, Deserialize};
use super::common::*;
use super::dpt;

mod run;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum State {
    Available,
    ExecutionLaunch,
    ExecutionProgress,
    ExecutionExit(u32),
    PostInit,
    PostProgress(Progress),
    PostComplete(Result<(),dpt::Error>),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Command {
    /// Sends DPT address.
    /// XCT posts execution result to DPT.
    /// This messages can be sent multiple times and XCT should use latest one.
    DPT(String),
    /// Ask a job execution.
    Ask(ID,Job),
    /// Stop and cancel on-going job.
    Halt,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Report {
    /// Accepts or rejects an offered job.
    /// XCT can rejct the job if it is not executable on current machine.
    Reply(ID, bool),
    /// Right before process launch.
    /// ARB regard this as "Launch OK."
    ExecutionLaunch,
    ExecutionProgress(StandardStreamKind, ByteChunk),
    /// Process exit code as-is.
    /// ARB regard this as "Execution OK."
    ExecutionExit(u32),

    PostInit,
    PostProgress(Progress),
    /// Signals posting to DPT done.
    /// ARB regard this as "Post OK."
    PostComplete(Result<(),dpt::Error>),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum StandardStreamKind {
    STDOUT, STDERR,
}

