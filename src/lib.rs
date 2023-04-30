use std::io::{ErrorKind, Read, Result, Write};

pub trait ReadSizedExt {
  fn read_sized(&mut self) -> Result<Vec<u8>>;
}

#[async_trait::async_trait]
pub trait AsyncReadSized {
  async fn aread_sized(&mut self) -> Result<Vec<u8>>;
}

impl<T: Read> ReadSizedExt for T {
  fn read_sized(&mut self) -> Result<Vec<u8>> {
    let mut size_buf = [0u8; std::mem::size_of::<usize>()];
    self.read_exact(&mut size_buf)?;
    let size = usize::from_be_bytes(size_buf);
    if size > 4_000_000 {
      return Err(std::io::Error::new(
        ErrorKind::InvalidData,
        "Too many bytes to allocate",
      ));
    }
    let mut result = vec![0u8; size];
    self.read_exact(&mut result)?;
    Ok(result)
  }
}

#[async_trait::async_trait]
impl<T: tokio::io::AsyncReadExt + Unpin + Send> AsyncReadSized for T {
  async fn aread_sized(&mut self) -> Result<Vec<u8>> {
    let mut size_buf = [0u8; std::mem::size_of::<usize>()];
    self.read_exact(&mut size_buf).await?;
    let size = usize::from_be_bytes(size_buf);
    if size > 4_000_000 {
      return Err(std::io::Error::new(
        ErrorKind::InvalidData,
        "Too many bytes to allocate",
      ));
    }
    let mut result = vec![0u8; size];
    self.read_exact(&mut result).await?;
    Ok(result)
  }
}
pub trait WriteSizedExt {
  fn write_sized(&mut self, data: impl AsRef<[u8]>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait AsyncWriteSized {
  async fn awrite_sized(&mut self, data: impl AsRef<[u8]> + Send) -> Result<()>;
}

impl<T: Write> WriteSizedExt for T {
  fn write_sized(&mut self, data: impl AsRef<[u8]>) -> Result<()> {
    let data = data.as_ref();
    self.write_all(&data.len().to_be_bytes())?;
    self.write_all(data)?;
    Ok(())
  }
}

#[async_trait::async_trait]
impl<T: tokio::io::AsyncWriteExt + Unpin + Send> AsyncWriteSized for T {
  async fn awrite_sized(&mut self, data: impl AsRef<[u8]> + Send) -> Result<()> {
    let data = data.as_ref();
    self.write_all(&data.len().to_be_bytes()).await?;
    self.write_all(data).await?;
    Ok(())
  }
}