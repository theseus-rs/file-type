use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009440: FileFormat = FileFormat {
    id: 28_009_440,
    source_type: SourceType::Wikidata,
    name: "PCAPNG",
    extensions: &["pcapng"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
