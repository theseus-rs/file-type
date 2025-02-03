use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2505: FileFormat = FileFormat {
    id: 2_505,
    source_type: SourceType::Pronom,
    name: "Roxio Data Project File",
    extensions: &["rox"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(160),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x39, 0x43, 0x41, 0x30, 0x45, 0x45, 0x45, 0x45, 0x2D, 0x35, 0x42, 0x43, 0x35,
                    0x2D, 0x34, 0x31, 0x65, 0x39, 0x2D, 0x38, 0x32, 0x34, 0x32, 0x2D, 0x42, 0x45,
                    0x45, 0x32, 0x31, 0x36, 0x34, 0x33, 0x46, 0x46, 0x46, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_502,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_503,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_504,
        },
    ],
};
