use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_858: FileFormat = FileFormat {
    id: 858,
    source_type: SourceType::Pronom,
    name: "Revit Family Template",
    extensions: &["rft"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 767,
    }],
};
