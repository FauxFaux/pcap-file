//! This module contains the `PcapReader` struct which is used to read from a pcap file

use byteorder::{BigEndian, LittleEndian};

use errors::*;

use packet::Packet;
use pcap_header::{PcapHeader, Endianness};

use peek_reader::PeekReader;

use std::io::Read;


/// This struct wraps another reader and enables it to read a Pcap formated stream.
///
/// It implements the Iterator trait in order to read one packet at a time
///
/// # Examples
///
/// ```rust,no_run
/// use std::fs::File;
/// use pcap_file::{PcapReader, PcapWriter};
///
/// let file_in = File::open("test.pcap").expect("Error opening file");
/// let pcap_reader = PcapReader::new(file_in).unwrap();
///
/// let file_out = File::create("out.pcap").expect("Error creating file");
/// let mut pcap_writer = PcapWriter::new(file_out).unwrap();
///
/// // Read test.pcap
/// for pcap in pcap_reader {
///
///     //Check if there is no error
///     let pcap = pcap.unwrap();
///
///     //Write each packet of test.pcap in out.pcap
///     pcap_writer.write_packet(&pcap).unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct PcapReader<T: Read> {

    pub header: PcapHeader,
    reader: PeekReader<T>
}

impl <T:Read> PcapReader<T>{

    /// Create a new PcapReader from an existing reader.
    /// This function read the global pcap header of the file to verify its integrity.
    ///
    /// The underlying reader must point to a valid pcap file/stream.
    ///
    /// # Errors
    /// Return an error if the data stream is not in a valid pcap file format.
    /// Or if the underlying data are not readable.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use std::fs::File;
    /// use pcap_file::PcapReader;
    ///
    /// let file_in = File::open("test.pcap").expect("Error opening file");
    /// let pcap_reader = PcapReader::new(file_in).unwrap();
    /// ```
    pub fn new(mut reader:T) -> ResultChain<PcapReader<T>> {

        Ok(
            PcapReader {

                header : PcapHeader::from_reader(&mut reader)?,
                reader : PeekReader::new(reader)
            }
        )
    }

    /// Consumes the `PcapReader`, returning the wrapped reader.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use std::fs::File;
    /// use pcap_file::PcapReader;
    ///
    /// let file = File::open("test.pcap").expect("Error opening file");
    /// let pcap_reader = PcapReader::new(file).unwrap();
    ///
    /// let file2 = pcap_reader.into_reader();
    /// ```
    pub fn into_reader(self) -> T{
        self.reader.inner
    }


    /// Gets a reference to the underlying reader.
    ///
    /// It is not advised to directly read from the underlying reader.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use std::fs::File;
    /// use pcap_file::PcapReader;
    ///
    /// let file = File::open("test.pcap").expect("Error opening file");
    /// let pcap_reader = PcapReader::new(file).unwrap();
    ///
    /// let file_ref = pcap_reader.get_ref();
    /// ```
    pub fn get_ref(&self) -> &T{
        &self.reader.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is not advised to directly read from the underlying reader.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use std::fs::File;
    /// use pcap_file::PcapReader;
    ///
    /// let file = File::open("test.pcap").expect("Error opening file");
    /// let mut pcap_reader = PcapReader::new(file).unwrap();
    ///
    /// let file_mut = pcap_reader.get_mut();
    /// ```
    pub fn get_mut(&mut self) -> &mut T{
        &mut self.reader.inner
    }
}

impl <T:Read> Iterator for PcapReader<T> {

    type Item = ResultChain<Packet<'static>>;

    fn next(&mut self) -> Option<ResultChain<Packet<'static>>> {

        match self.reader.is_empty() {
            Ok(is_empty) if is_empty => {
                return None;
            },
            Err(err) => return Some(Err(err.into())),
            _ => {}
        }

        Some(
            match self.header.endianness() {
                Endianness::Big => Packet::from_reader::<_, BigEndian>(&mut self.reader),
                Endianness::Little => Packet::from_reader::<_, LittleEndian>(&mut self.reader)
            }
        )
    }

}