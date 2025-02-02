use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_848: FileFormat = FileFormat {
    id: 848,
    source_type: SourceType::Pronom,
    name: "CATIA Project",
    extensions: &["project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
