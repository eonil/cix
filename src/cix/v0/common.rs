use serde::{Serialize, Deserialize};

pub type ID = String;
pub type Path = String;
pub type URL = String;
pub type ByteChunk = Vec<u8>;
pub type ByteRange = std::ops::Range<u64>;
pub type SHA1Hash = String;

/// Defines a job.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Job {
    repo: URL,
    commit: String,
    action: JobAction,
}

/// Defines what to do with job source code.
/// Some actions post resulting artifacts to DPT.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum JobAction {
    /// Builds Xcode Workspace Scheme.
    /// This always performs clean build.
    XcodeBuild {
        workspace: Path,
        scheme: String,
        configuration: String,
        /// Whether to deploy built product to DPT or not.
        deploy: bool,
    },
    /// Runs unit tests designated in Xcode Test Plan of designated Scheme.
    /// This produces test coverage report and posts the report to DPT.
    XcodeUnitTest {
        workspace: Path,
        scheme: String,
        testplan: String,
        /// Platforms to test against.
        platforms: Vec<XcodeTestPlatform>,
    },
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum XcodeTestPlatform {
    Simulator { hardware: String, software: String },
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum FileMessage {
    Metadata { id: String, len: u64 },
    /// Part of file content.
    /// This must be sent sequentially.
    Content(ByteRange, ByteChunk),
    /// SHA1 hash to verify integrity of transferred data as whole.
    VerificationSHA1(SHA1Hash),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum ArtifactKind {
    /// Apple `.ipa` package.
    AppleIPA,
    /// Apple zipped `.app` package.
    AppleAppZip,
    /// Apple debug symbol package.
    AppleDSYMZip,
    /// Xcode unit test coverage report file.
    AppleXcodeTestCoverage,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Progress { 
    length: u64, 
    point: u64,
}