use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485505: FileFormat = FileFormat {
    id: 117_485_505,
    source_type: SourceType::Wikidata,
    name: "MacCaption Project",
    extensions: &["cca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
