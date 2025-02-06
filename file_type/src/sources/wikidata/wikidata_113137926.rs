use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113137926: FileFormat = FileFormat {
    id: 113_137_926,
    source_type: SourceType::Wikidata,
    name: "Wireshark nanosecond libpcap",
    extensions: &["cap", "pcap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
