use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_469: FileFormat = FileFormat {
    id: 469,
    source_type: SourceType::Pronom,
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
