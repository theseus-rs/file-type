use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_156: FileFormat = FileFormat {
    id: 799,
    puid: "fmt/156",
    name: "Tagged Image File Format for Internet Fax (TIFF-FX)",
    extensions: &["tif", "tiff", "tfx"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 672,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 673,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 752,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_099,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 612,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
