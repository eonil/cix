//! Messages between ARB UI server and client.

use common::*;
use xct;

enum Command {
    AskKnownXCTAddressList,
    AskRunningXCTAddressList,
    AskRunningXCTState,
    AskXCTState(URL),
    StreamXCTReport(URL),

    AddXCTOverSSH(URL),
    RemoveXCT(URL),
}

enum Report {
    ReplyKnownXCTAddressList(Vec<URL>),
    ReplyRunningXCTAddressList(Vec<URL>),
    ReplyRunningXCTState(URL,XCTState),
    XCTReport(URL,xct::Report),
}
