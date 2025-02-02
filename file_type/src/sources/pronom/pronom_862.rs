use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_862: FileFormat = FileFormat {
    id: 862,
    source_type: SourceType::Pronom,
    name: "Revit Project",
    extensions: &["rvt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 767,
    }],
};
