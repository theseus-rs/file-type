use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859382: FileFormat = FileFormat {
    id: 105_859_382,
    source_type: SourceType::Wikidata,
    name: "QTI document",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
