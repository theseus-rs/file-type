use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28955450: FileFormat = FileFormat {
    id: 28_955_450,
    source_type: SourceType::Wikidata,
    name: "GenBank file format",
    extensions: &["gb", "gbk", "gen"],
    media_types: &["chemical/x-genbank"],
    signatures: &[],
    related_formats: &[],
};
