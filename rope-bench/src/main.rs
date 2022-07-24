use std::future::Future;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};

use ansi_term::Color::{Blue, Green, Red};
use ansi_term::Style;
use anyhow::{ensure, Context, Result};
use clap::Parser;
use indoc::formatdoc;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use redis::aio::Connection;
use redis::{AsyncCommands, Client, FromRedisValue};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::time::{self, Duration, Instant};

#[derive(Parser)]
#[clap(about, long_about = None)]
/// A fast benchmarking harness for the redis-rope module.
struct Args {
    /// Path to the redisrope module shared library.
    #[clap(value_parser)]
    module_path: PathBuf,

    /// Unix domain socket for redis connections.
    #[clap(short, long, value_parser, default_value = "/tmp/redis.sock")]
    socket: PathBuf,

    /// Set to hide output from the Redis server.
    #[clap(short, long)]
    quiet: bool,
}

/// Spawns a redis server at the location.
async fn spawn_server(args: &Args) -> Result<Child> {
    fs::remove_file(&args.socket).await.ok();

    let get_output = || {
        if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        }
    };
    let mut child = Command::new("redis-server")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(get_output())
        .stderr(get_output())
        .kill_on_drop(true)
        .spawn()?;
    {
        let mut stdin = child.stdin.take().unwrap();
        let options = formatdoc! {"
            save \"\"
            dbfilename \"\"
            port 0
            unixsocket {}
            loadmodule {}",
            args.socket.display(),
            args.module_path.display(),
        };
        stdin.write_all(options.as_bytes()).await?;
    }

    // Wait for redis to start.
    while fs::metadata(&args.socket).await.is_err() {
        time::sleep(Duration::from_millis(5)).await;
    }

    Ok(child)
}

/// Sends a termination signal to a child process and waits for it.
async fn terminate(mut child: Child) -> Result<ExitStatus> {
    signal::kill(Pid::from_raw(child.id().unwrap() as i32), Signal::SIGTERM)?;
    Ok(child.wait().await?)
}

/// Retrieves the name of a function.
fn function_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

async fn run_test<F, Fut>(client: &Client, func: F) -> Result<()>
where
    F: Fn(Client) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let name = function_name(&func);
    print!("{name} ... ");

    let start = Instant::now();
    let result = func(client.clone()).await;
    let duration = format!("({:?})", start.elapsed());
    let duration = Style::new().dimmed().paint(duration);

    match result {
        Ok(()) => println!("{}  {}", Green.paint("ok!"), duration),
        Err(err) => {
            println!("{}  {}", Red.paint("ERR"), duration);
            println!("{:?}", err);
            return Err(err);
        }
    }
    Ok(())
}

async fn query<T: FromRedisValue>(cmd: &str, conn: &mut Connection) -> Result<T> {
    let mut iter = cmd.split(' ');
    let start = iter.next().context("command is empty")?;
    let mut builder = redis::cmd(start);
    for arg in iter {
        builder.arg(arg);
    }
    Ok(builder.query_async(conn).await?)
}

async fn basic_ops(client: Client) -> Result<()> {
    let conn = &mut client.get_async_connection().await?;

    conn.set("hello", "world").await?;
    ensure!(conn.get::<_, String>("hello").await? == "world");
    conn.del("hello").await?;

    Ok(())
}

async fn rope_len(client: Client) -> Result<()> {
    let conn = &mut client.get_async_connection().await?;

    let result: i64 = query("ROPE.LEN hello", conn).await?;
    ensure!(result == 0);

    conn.set("hello", "world").await?;
    let result = query::<i64>("ROPE.LEN hello", conn).await;
    ensure!(result.is_err());
    ensure!(result.unwrap_err().to_string().contains("WRONGTYPE"));
    conn.del("hello").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let child = spawn_server(&args).await?;

    {
        let uri = format!("redis+unix:///{}", args.socket.display());
        let client = Client::open(uri)?;

        println!("{}", Blue.paint("------ STARTING TESTS ------"));
        run_test(&client, basic_ops).await?;
        run_test(&client, rope_len).await?;
        println!("{}", Blue.paint("----- ALL TESTS PASSED -----"));
    }

    terminate(child).await?;
    Ok(())
}
