use std::io::Read;
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Batas output per perintah agar UI tidak kebanjiran.
pub const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const TIMEOUT: Duration = Duration::from_secs(10);
const POLL: Duration = Duration::from_millis(20);

/// Abstraksi eksekusi CLI docker; implementasi uji cukup mengembalikan fixture.
pub trait DockerCli: Send + Sync {
    fn run(&self, args: &[&str]) -> Result<String, String>;
}

/// Implementasi nyata memakai binary `docker`, argumen array tanpa `sh -c`.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemDockerCli;

impl DockerCli for SystemDockerCli {
    fn run(&self, args: &[&str]) -> Result<String, String> {
        let mut child = Command::new("docker")
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("docker tidak dapat dijalankan: {error}"))?;

        let stdout = child.stdout.take().map(spawn_reader);
        let stderr = child.stderr.take().map(spawn_reader);

        let started = Instant::now();
        let mut timed_out = false;
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) if started.elapsed() >= TIMEOUT => {
                    let _ = child.kill();
                    let _ = child.wait();
                    timed_out = true;
                    break None;
                }
                Ok(None) => thread::sleep(POLL),
                Err(error) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("docker gagal dipantau: {error}"));
                }
            }
        };

        let out = join(stdout);
        let err = join(stderr);
        if timed_out {
            return Err("perintah docker melewati batas 10 detik".to_owned());
        }
        let status = status.expect("status ada bila tidak timeout");
        if !status.success() {
            let message = String::from_utf8_lossy(&err).trim().to_owned();
            return Err(if message.is_empty() {
                format!("docker keluar dengan status {status}")
            } else {
                message
            });
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }
}

/// Baca sampai EOF, simpan hanya `MAX_OUTPUT_BYTES` pertama agar pipe tetap kosong.
fn spawn_reader(mut stream: impl Read + Send + 'static) -> JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut stored = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(read) => {
                    let room = MAX_OUTPUT_BYTES.saturating_sub(stored.len());
                    stored.extend_from_slice(&buf[..read.min(room)]);
                }
                Err(_) => break,
            }
        }
        stored
    })
}

fn join(handle: Option<JoinHandle<Vec<u8>>>) -> Vec<u8> {
    handle
        .map(|handle| handle.join().unwrap_or_default())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn reader_caps_stored_bytes() {
        let data = vec![b'x'; MAX_OUTPUT_BYTES + 1000];
        let stored = spawn_reader(Cursor::new(data)).join().expect("reader");
        assert_eq!(stored.len(), MAX_OUTPUT_BYTES);
    }
}
