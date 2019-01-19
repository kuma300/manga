pub mod epub;
pub mod pdf;
pub mod prelude;

use crate::{check, errors::*, fetch::*};

use std::process::{Command, ExitStatus, Stdio};

pub fn book_convert(src: &str, dst: &str) -> Result<()> {
    let program = "ebook-convert";
    if !check::exec_succeed(program, &["--version"]) {
        return Err(err_msg(
            "please install Calibre: https://calibre-ebook.com/download",
        ));
    }

    let status = Command::new(program)
        .arg(&src)
        .arg(&dst)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(err_msg(format!(
            "possible conversion failed with an incorrect exit code: {}",
            status.code().ok_or(err_msg(
                "possible conversion failed and no exit code was obtained"
            ))?
        )))
    }
}
