use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113137926: FileFormat = FileFormat {
    id: 113_137_926,
    puid: "wikidata/113137926",
    name: "Wireshark nanosecond libpcap",
    extensions: &["cap", "pcap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
