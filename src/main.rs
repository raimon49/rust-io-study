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

        fn grep<R>(target: &str, reader: R) -> io::Result<()>
            where R: BufRead
        {
            for line_result in reader.lines() {
                let line = line_result?; // io::Result<String>のエラーチェックして取り出し
                if line.contains(target) {
                    println!("{}", line);
                }
            }
            Ok(())
        }

    // stdinは排他ロックでガードされているため.lock()でBufReadを実装したStdinLockを取得する
    // let stdin = io::stdin();
    // grep(&target, stdin.lock())?;

    // FileはBufReadを実装していないがBufReader::new()に渡せばバッファリングしながら読み出せる
    // let f = File::open(file)?;
    // grep(&target, BufReader::new(f))?;
    }
    {
        use std::io;
        use std::io::prelude::*;

        fn line_to_vec_with_loop<R>(reader: R)
            where R: BufRead
        {
            // 1行ずつベクタに格納する
            let mut lines_with_loop = vec![];
            for line_result in reader.lines() {
                let line = line_result;
                lines_with_loop.push(line);
            }

        }
        fn line_to_vec_with_collect<R>(reader: R)
            where R: BufRead
        {
            // collect()で格納する
            let lines_with_collect = reader.lines().collect::<io::Result<Vec<String>>>();
        }
    }
    {
        eprintln!("error: world note helloable");
    }
    {
        use std::fs::OpenOptions;

        fn _open() {
            // ファイルが存在したら、後ろに追記する
            let log = OpenOptions::new()
                .append(true)
                .open("/tmp/tmp.log");
        }

        fn _write() {
            // ファイルが存在したら、失敗させる
            let log = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("/tmp/tmp.file");
        }
    }
    {
        use std::io::Read;
        use std::io;

        let mut buf = String::new();
        let mut e = io::empty(); // 読み出しは常に成功するが何も返さないreader実装
        e.read_to_string(&mut buf).unwrap();
        assert!(buf.is_empty());
    }
}
