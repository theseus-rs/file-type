use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2506: FileFormat = FileFormat {
    id: 2_506,
    source_type: SourceType::Pronom,
    name: "Roxio Audio Project File",
    extensions: &["rox"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(160),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0x30, 0x32, 0x35, 0x33, 0x43, 0x34, 0x43, 0x2D, 0x32, 0x32, 0x39, 0x44,
                    0x2D, 0x34, 0x63, 0x38, 0x37, 0x2D, 0x38, 0x44, 0x31, 0x44, 0x2D, 0x31, 0x36,
                    0x39, 0x45, 0x46, 0x44, 0x46, 0x45, 0x44, 0x38, 0x36, 0x39,
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
            id: 2_503,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_504,
        },
    ],
};
