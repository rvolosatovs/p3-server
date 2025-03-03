use futures::{SinkExt as _, StreamExt as _, TryStreamExt as _};
use wasi::sockets::types::TcpSocket;
use wasi::wit_stream;

struct Component;

wasi::cli::command::export!(Component);

impl wasi::exports::cli::run::Guest for Component {
    async fn run() -> Result<(), ()> {
        let args = wasi::cli::environment::get_arguments();
        let [_, port] = args.as_slice() else {
            return Ok(());
        };
        let port = port.parse().expect("failed to parse port");

        let (mut stdout_tx, stdout_rx) = wit_stream::new();
        wasi::cli::stdout::set_stdout(stdout_rx);
        stdout_tx.send(b"hello stdout\n".into()).await.or(Err(()))?;

        let (mut stderr_tx, stderr_rx) = wit_stream::new();
        wasi::cli::stderr::set_stderr(stderr_rx);
        stderr_tx.send(b"hello stderr\n".into()).await.or(Err(()))?;

        let sock = TcpSocket::new(wasi::sockets::types::IpAddressFamily::Ipv4);
        sock.bind(wasi::sockets::types::IpSocketAddress::Ipv4(
            wasi::sockets::types::Ipv4SocketAddress {
                address: (127, 0, 0, 1),
                port,
            },
        ))
        .expect("failed to bind");
        let mut accept = sock.listen().expect("failed to listen");
        while let Ok(accepted) = accept.next().await.unwrap() {
            for sock in accepted {
                let (data_rx, fut) = sock.receive();
                let data = data_rx.try_collect::<Vec<_>>().await.unwrap().concat();
                stdout_tx.send(data).await.or(Err(()))?;

                fut.await.unwrap().unwrap().unwrap();
            }
        }
        Ok(())
    }
}
