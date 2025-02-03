use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117187116: FileFormat = FileFormat {
    id: 117_187_116,
    source_type: SourceType::Wikidata,
    name: "CD Stomper Template file",
    extensions: &["dsu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
