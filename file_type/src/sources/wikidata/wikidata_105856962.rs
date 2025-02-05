use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856962: FileFormat = FileFormat {
    id: 105_856_962,
    source_type: SourceType::Wikidata,
    name: "Game Description Language (with rem)",
    extensions: &["gdl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
