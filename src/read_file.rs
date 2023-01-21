use std::fs;
use std::io;
use std::io::Read;

pub fn _read_basic() -> io::Result<()> {
    let _read_basic = fs::read_to_string("./some_poems.txt").expect("cant read files or cant find files");

    const FILE_PATH: &str = "./japanese_news.txt";

    let _error_handle_read_file = match fs::File::open(FILE_PATH) {
        Ok(_news) => _news,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("file not founded");
                    return Ok(())
                }

                _ => return Err(error)
            }
        }
    };

    
    let _read_some_news = fs::read_to_string(FILE_PATH).expect("cant read files");

    println!("{}", _read_some_news);

    Ok(())
}

pub fn _file_content() -> io::Result<()> {
    let mut _file_contents = match fs::File::open("./some_opems.txt") {
        Ok(_file_contents) => _file_contents,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("file not founded");
                    return Ok(())
                }

                _ => return Err(error)
            }
        }
    };
    let mut contents = Vec::new();
    _file_contents.read_to_end(&mut contents)?;

    print!("File content : {:?}", contents);
    Ok(())
}