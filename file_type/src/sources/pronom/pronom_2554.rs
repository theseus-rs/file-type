use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2554: FileFormat = FileFormat {
    id: 2_554,
    source_type: SourceType::Pronom,
    name: "PageMaker Mac Document",
    extensions: &["p65", "t65", "pmd", "pmt"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 254,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_680,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 247,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_555,
        },
    ],
};
