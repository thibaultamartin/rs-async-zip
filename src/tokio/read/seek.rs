// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A ZIP reader which acts over a seekable source.

use crate::base::read::seek::ZipFileReader as BaseZipFileReader;
use crate::error::Result;
use crate::file::ZipFile;
use crate::tokio::read::ZipEntryReader;

use tokio::io::{AsyncRead, AsyncSeek};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

/// A ZIP reader which acts over a seekable source.
#[derive(Clone)]
pub struct ZipFileReader<R>(BaseZipFileReader<Compat<R>>);

impl<R> ZipFileReader<R>
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    /// Constructs a new ZIP reader from a seekable source.
    pub async fn new(reader: R) -> Result<ZipFileReader<R>> {
        Ok(ZipFileReader(BaseZipFileReader::new(reader.compat()).await?))
    }

    /// Constructs a ZIP reader from a seekable source and ZIP file information derived from that source.
    ///
    /// Providing a [`ZipFile`] that wasn't derived from that source may lead to inaccurate parsing.
    pub fn from_raw_parts(reader: R, file: ZipFile) -> ZipFileReader<R> {
        ZipFileReader(BaseZipFileReader::from_raw_parts(reader.compat(), file))
    }

    /// Returns this ZIP file's information.
    pub fn file(&self) -> &ZipFile {
        self.0.file()
    }

    /// Returns a mutable reference to the inner seekable source.
    ///
    /// Swapping the source (eg. via std::mem operations) may lead to inaccurate parsing.
    pub fn inner_mut(&mut self) -> &mut R {
        self.0.inner_mut().get_mut()
    }

    /// Returns the inner seekable source by consuming self.
    pub fn into_inner(self) -> R {
        self.0.into_inner().into_inner()
    }

    /// Returns a new entry reader if the provided index is valid.
    pub async fn entry(&mut self, index: usize) -> Result<ZipEntryReader<'_, R>> {
        Ok(ZipEntryReader(self.0.entry(index).await?))
    }

    /// Returns a new entry reader if the provided index is valid.
    /// Consumes self
    pub async fn into_entry<'a>(self, index: usize) -> Result<ZipEntryReader<'a, R>>
    where
        R: 'a,
    {
        Ok(ZipEntryReader(self.0.into_entry(index).await?))
    }
}
