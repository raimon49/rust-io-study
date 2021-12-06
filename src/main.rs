fn main() {
    {
        // selfキーワードは「io」をstd::ioの別名として使う宣言
        use std::io::{self, Read, Write, ErrorKind};

        const DEFAULT_BUF_SIZE: usize = 8 * 1024;

        // std::io::copy()の実装
        // ジェネリックに実装されているためFileからTcpStreamへコピーするのも、Stdinからメモリ上のVec<u8>へコピーするのもできる
        pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
            where R: Read, W: Write
        {
            let mut buf = [0; DEFAULT_BUF_SIZE];
            let mut written = 0;
            loop {
                let len = match reader.read(&mut buf) {
                    Ok(0) => return Ok(written),
                    Ok(len) => len,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                writer.write_all(&buf[..len])?;
                written += len as u64;
            }
        }
    }
    {
        use std::io;
        use std::io::prelude::*;

        fn grep(target: &str) -> io::Result<()> {
            let stdin = io::stdin();
            for line_result in stdin.lock().lines() {
                let line = line_result?;
                if line.contains(target) {
                    println!("{}", line);
                }
            }
            Ok(())
        }
    }
}
