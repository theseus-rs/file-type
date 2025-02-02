use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2043681: FileFormat = FileFormat {
    id: 2_043_681,
    source_type: SourceType::Wikidata,
    name: "PAK",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
