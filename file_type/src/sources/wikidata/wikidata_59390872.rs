use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59390872: FileFormat = FileFormat {
    id: 59_390_872,
    source_type: SourceType::Wikidata,
    name: "GraphPad Prism file format",
    extensions: &["pzm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
