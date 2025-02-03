use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_547: FileFormat = FileFormat {
    id: 547,
    source_type: SourceType::Pronom,
    name: "XYWrite for Windows Document",
    extensions: &["xyw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
