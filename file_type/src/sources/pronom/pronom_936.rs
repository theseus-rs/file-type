use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_936: FileFormat = FileFormat {
    id: 936,
    source_type: SourceType::Pronom,
    name: "Statistica Report File",
    extensions: &["str"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
