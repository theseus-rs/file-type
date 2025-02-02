use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_469: FileFormat = FileFormat {
    id: 469,
    source_type: SourceType::Pronom,
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
