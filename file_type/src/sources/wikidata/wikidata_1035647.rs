use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1035647: FileFormat = FileFormat {
    id: 1_035_647,
    source_type: SourceType::Wikidata,
    name: "Card Verifiable Certificate",
    extensions: &["cv", "cvcert"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
