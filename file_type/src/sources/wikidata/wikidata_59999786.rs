use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59999786: FileFormat = FileFormat {
    id: 59_999_786,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Lock File",
    extensions: &["lck"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
