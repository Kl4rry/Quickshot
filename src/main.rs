use ashpd::desktop::screenshot::Screenshot;
use std::{pin::Pin, process::Stdio};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub fn get_clap_command() -> clap::Command {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("modal")
                .short('m')
                .long("modal")
                .help("Whether the dialog should be modal")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Customize area before taking a screenshot")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .help("Output filepath for screenshot"),
        )
        .arg(
            clap::Arg::new("pipe")
                .short('p')
                .long("pipe")
                .help("Send image data over stdout")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("exec")
                .short('e')
                .long("exec")
                .num_args(1..)
                .help("Spawn child and pipe output to child"),
        )
        .arg(
            clap::Arg::new("generate-man")
                .long("generate-man")
                .help("Generates manual page for quickshot")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("wait")
                .short('w')
                .long("wait")
                .help("Wait for child to exit and return exit status of child")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't print path to stdout")
                .action(clap::ArgAction::SetTrue),
        )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = get_clap_command().get_matches();

    if args.get_flag("generate-man") {
        let man = clap_mangen::Man::new(get_clap_command());
        let mut buffer: Vec<u8> = Vec::new();
        man.render(&mut buffer)?;
        tokio::fs::write("quickshot.1", buffer).await?;
        return Ok(());
    }

    let response = Screenshot::request()
        .interactive(args.get_flag("interactive"))
        .modal(args.get_flag("modal"))
        .send()
        .await?
        .response()?;

    let mut child = None;

    if let Some(values) = args.get_many::<String>("exec") {
        let exec: Vec<&String> = values.collect();
        child = tokio::process::Command::new(exec[0])
            .args(&exec[1..])
            .stdin(Stdio::piped())
            .spawn()?
            .into();
    }

    let mut output: Pin<Box<dyn AsyncWrite>> = if let Some(ref mut child) = child {
        Box::pin(child.stdin.take().unwrap())
    } else {
        Box::pin(tokio::io::stdout())
    };

    let tmp_path = response.uri().path();
    let path = if let Some(path) = args.get_one::<String>("output") {
        tokio::fs::copy(tmp_path, path).await?;
        path
    } else {
        tmp_path
    };

    if args.get_flag("pipe") {
        let mut file = tokio::fs::File::open(response.uri().path()).await?;
        tokio::io::copy(&mut file, &mut output).await?;
    } else if !args.get_flag("quiet") {
        output.write_all(path.as_bytes()).await?;
    }

    if args.get_flag("wait") {
        if let Some(mut child) = child {
            let status = child.wait().await?;
            std::process::exit(status.code().unwrap_or(1));
        }
    }
    Ok(())
}
