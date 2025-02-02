use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111292287: FileFormat = FileFormat {
    id: 111_292_287,
    source_type: SourceType::Wikidata,
    name: "linux shared library",
    extensions: &["so"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
