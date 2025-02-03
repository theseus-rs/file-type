use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119973449: FileFormat = FileFormat {
    id: 119_973_449,
    source_type: SourceType::Wikidata,
    name: "WebEasy Web Document",
    extensions: &["web"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
