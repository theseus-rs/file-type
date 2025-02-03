use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113137926: FileFormat = FileFormat {
    id: 113_137_926,
    source_type: SourceType::Wikidata,
    name: "Wireshark nanosecond libpcap",
    extensions: &["cap", "pcap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
