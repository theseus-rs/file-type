use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009435: FileFormat = FileFormat {
    id: 28_009_435,
    source_type: SourceType::Wikidata,
    name: "PCAP",
    extensions: &["pcap"],
    media_types: &["application/vnd.tcpdump.pcap"],
    signatures: &[],
    related_formats: &[],
};
