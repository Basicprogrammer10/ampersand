use std::io::{self, Read, Write};

pub struct Socket {
    buffer: Vec<u8>,
    open: bool,
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.open {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Socket is closed",
            ));
        }

        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.open {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Socket is closed",
            ));
        }

        if self.buffer.is_empty() && self.open {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "No data available",
            ));
        }

        let len = buf.len().min(self.buffer.len());
        buf[..len].copy_from_slice(&self.buffer[..len]);
        self.buffer.drain(..len);
        Ok(len)
    }
}
