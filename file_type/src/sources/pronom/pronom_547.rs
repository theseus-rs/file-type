use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_547: FileFormat = FileFormat {
    id: 547,
    source_type: SourceType::Pronom,
    name: "XYWrite for Windows Document",
    extensions: &["xyw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
