use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1756: FileFormat = FileFormat {
    id: 2_604,
    puid: "fmt/1756",
    name: "OpenDocument Text",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_033,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
