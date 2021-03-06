use std::fs;
use std::io::{BufRead, BufReader, Error as IoError, Seek, SeekFrom};
use std::path::Path;

pub trait LogReader {
    fn seek(&mut self, pos: u64) -> Result<(), IoError>;
    fn tell(&self) -> u64;
    fn read_record(&mut self) -> Result<Option<String>, IoError>;
}

pub struct LogFile {
    pub file: BufReader<fs::File>,
    pub pos: u64,
}

impl LogFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<LogFile, IoError> {
        Ok(LogFile {
            file: BufReader::new(fs::File::open(path)?),
            pos: 0,
        })
    }
}

impl LogReader for LogFile {
    fn seek(&mut self, pos: u64) -> Result<(), IoError> {
        self.file.seek(SeekFrom::Start(pos))?;
        Ok(())
    }

    fn tell(&self) -> u64 {
        self.pos
    }

    fn read_record(&mut self) -> Result<Option<String>, IoError> {
        let mut line = String::new();
        let ret = self.file.read_line(&mut line)?;
        if ret == 0 {
            Ok(None)
        } else {
            self.pos += ret as u64;
            if line.len() >= 2 && line.ends_with("\r\n") {
                line.pop();
                line.pop();
            } else if line.len() >= 1 && line.ends_with("\n") {
                line.pop();
            }
            Ok(Some(line))
        }
    }
}
