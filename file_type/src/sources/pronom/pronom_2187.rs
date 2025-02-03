use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2187: FileFormat = FileFormat {
    id: 2_187,
    source_type: SourceType::Pronom,
    name: "Error File",
    extensions: &["err"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
