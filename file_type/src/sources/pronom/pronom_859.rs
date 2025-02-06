use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_859: FileFormat = FileFormat {
    id: 859,
    source_type: SourceType::Pronom,
    name: "Revit Template",
    extensions: &["rte"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 767,
    }],
};
