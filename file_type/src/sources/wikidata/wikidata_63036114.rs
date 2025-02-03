use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63036114: FileFormat = FileFormat {
    id: 63_036_114,
    source_type: SourceType::Wikidata,
    name: "8-bit ANSI Text",
    extensions: &["ans"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
