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
        use std::io::{self, Read, Write, ErrorKind};

        let mut buf = String::new();
        io::empty().read_to_string(&mut buf).unwrap(); // 読み出しは常に成功するが何も返さないreader実装
        assert!(buf.is_empty());

        let write_buf = vec![1, 2, 3, 5, 8];
        let bytes = io::sink().write(&write_buf).unwrap(); // 書き出しは常に成功するが何もしないwriter実装
        assert_eq!(bytes, 5);

        let mut repeat_buf = [0; 3]; // 指定したバイト値を繰り返すreader実装
        io::repeat(0b101).read_exact(&mut repeat_buf).unwrap();
        assert_eq!(repeat_buf, [0b101, 0b101, 0b101]);
    }
    {
        use std::collections::HashMap;
        use std::io;
        use serde::Serialize;
        use serde::Deserialize;
        use serde_json::Serializer;

        type RoomId = String;
        type RoomExits = Vec<(char, RoomId)>;
        type RoomMap = HashMap<RoomId, RoomExits>;

        let mut map = RoomMap::new();
        map.insert("Cobble Crawl".to_string(),
                   vec![('W', "Debris Room".to_string())]);
        map.insert("Debris Room".to_string(),
                   vec![('E', "Cobble Crawl".to_string()),
                        ('W', "Sloping Cayon".to_string())]);

        // serde::Serializeトレイトのserializeメソッドはシリアライズ方法を知っているすべてのデータ型で使える
        // （文字列、文字、タプル、ベクタ、HashMapなど）
        let mut serializer = Serializer::new(io::stdout());
        // 標準出力：{"Cobble Crawl":[["W","Debris Room"]],"Debris Room":[["E","Cobble Crawl"],["W","Sloping Cayon"]]}
        map.serialize(&mut serializer);
        println!("");

        #[derive(Serialize, Deserialize)]
        struct Player {
            location: String,
            items: Vec<String>,
            health: u32
        }

        let player = Player{
            location: "Cobble Crawl".to_string(),
            items: vec!["a wand".to_string()],
            health: 3
        };
        // 標準出力：{"location":"Cobble Crawl","items":["a wand"],"health":3}
        player.serialize(&mut serializer);
    }
    {
        use std::path::Path;

        assert_eq!(Path::new("/home/raimon49/.bashrc").parent(),
                   Some(Path::new("/home/raimon49")));
    }
}
