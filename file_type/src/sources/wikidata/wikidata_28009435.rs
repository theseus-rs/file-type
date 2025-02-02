use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009435: FileFormat = FileFormat {
    id: 28_009_435,
    source_type: SourceType::Wikidata,
    name: "PCAP",
    extensions: &["pcap"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
