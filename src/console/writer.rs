use std::io::Write;
use std::sync::Mutex;

use color_eyre::Result;
use flume::{Receiver, Sender};
use rustyline::ExternalPrinter;

pub struct ThreadWriter {
    tx: Sender<String>,
}

impl ThreadWriter {
    pub fn new() -> (Mutex<Self>, Receiver<String>) {
        let (tx, rx) = flume::unbounded();

        let writer = Self { tx };
        let writer = Mutex::new(writer);

        (writer, rx)
    }
}

impl Write for ThreadWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vec = buf.to_owned();
        if let Ok(string) = String::from_utf8(vec) {
            // TODO: Remove unwrap
            self.tx.send(string).unwrap();

            Ok(buf.len())
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub async fn write_loop(mut printer: impl ExternalPrinter, rx: Receiver<String>) -> Result<()> {
    while let Ok(msg) = rx.recv_async().await {
        printer.print(msg)?;
    }

    Ok(())
}
