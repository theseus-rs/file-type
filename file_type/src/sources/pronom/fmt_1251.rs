use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1251: FileFormat = FileFormat {
    id: 2_069,
    puid: "fmt/1251",
    name: "Electronically Certified Document (EDOC)",
    extensions: &["edoc"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_389,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_389,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
