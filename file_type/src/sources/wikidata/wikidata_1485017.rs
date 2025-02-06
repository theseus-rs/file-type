use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1485017: FileFormat = FileFormat {
    id: 1_485_017,
    source_type: SourceType::Wikidata,
    name: "GDSII stream format",
    extensions: &["gds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
