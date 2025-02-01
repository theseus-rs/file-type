use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1342: FileFormat = FileFormat {
    id: 2_160,
    puid: "fmt/1342",
    name: "BDOC",
    extensions: &["bdoc", "asice"],
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
