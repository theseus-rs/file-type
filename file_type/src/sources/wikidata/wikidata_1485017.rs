use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1485017: FileFormat = FileFormat {
    id: 1_485_017,
    source_type: SourceType::Wikidata,
    name: "GDSII stream format",
    extensions: &["gds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
