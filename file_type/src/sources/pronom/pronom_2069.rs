use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2069: FileFormat = FileFormat {
    id: 2_069,
    source_type: SourceType::Pronom,
    name: "Electronically Certified Document (EDOC)",
    extensions: &["edoc"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_389,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 2_389,
        },
    ],
};
