use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59390872: FileFormat = FileFormat {
    id: 59_390_872,
    source_type: SourceType::Wikidata,
    name: "GraphPad Prism file format",
    extensions: &["pzm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
