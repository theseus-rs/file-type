use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1349: FileFormat = FileFormat {
    id: 2_167,
    puid: "fmt/1349",
    name: "Autodesk Revit Family File",
    extensions: &["rfa", "rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_164,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_165,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_166,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
