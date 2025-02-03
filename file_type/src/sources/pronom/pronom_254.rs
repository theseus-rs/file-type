use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_254: FileFormat = FileFormat {
    id: 254,
    source_type: SourceType::Pronom,
    name: "PageMaker PC Document",
    extensions: &["p65", "t65", "pmd", "pmt"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 2_554,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_680,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 247,
        },
    ],
};
