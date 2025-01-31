use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009435: FileFormat = FileFormat {
    id: 28_009_435,
    puid: "wikidata/28009435",
    name: "PCAP",
    extensions: &["pcap"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
