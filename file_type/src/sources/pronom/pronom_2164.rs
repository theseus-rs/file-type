use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2164: FileFormat = FileFormat {
    id: 2_164,
    source_type: SourceType::Pronom,
    name: "Autodesk Revit File",
    extensions: &["rvt", "rfa", "rte", "rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_165,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_166,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_167,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_168,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_169,
        },
    ],
};
