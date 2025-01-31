use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1350: FileFormat = FileFormat {
    id: 2_168,
    puid: "fmt/1350",
    name: "Autodesk Revit Project File",
    extensions: &["rvt", "rte"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_169,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_164,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
