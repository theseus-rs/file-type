use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856962: FileFormat = FileFormat {
    id: 105_856_962,
    source_type: SourceType::Wikidata,
    name: "Game Description Language (with rem)",
    extensions: &["gdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
