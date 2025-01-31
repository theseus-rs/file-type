use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1718: FileFormat = FileFormat {
    id: 2_554,
    puid: "fmt/1718",
    name: "PageMaker Mac Document",
    extensions: &["p65", "t65", "pmd", "pmt"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 254,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 1_680,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 247,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_555,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
