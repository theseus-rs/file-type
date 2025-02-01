use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009440: FileFormat = FileFormat {
    id: 28_009_440,
    puid: "wikidata/28009440",
    name: "PCAPNG",
    extensions: &["pcapng"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
