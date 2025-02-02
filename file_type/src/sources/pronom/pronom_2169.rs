use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2169: FileFormat = FileFormat {
    id: 2_169,
    source_type: SourceType::Pronom,
    name: "Autodesk Revit Family File",
    extensions: &["rfa", "rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_164,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_168,
        },
    ],
};
