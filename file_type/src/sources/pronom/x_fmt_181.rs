use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_181: FileFormat = FileFormat {
    id: 254,
    puid: "x-fmt/181",
    name: "PageMaker PC Document",
    extensions: &["p65", "t65", "pmd", "pmt"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_554,
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
    ],
};
