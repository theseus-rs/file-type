use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2624: FileType = FileType {
    file_format: &FileFormat {
        id: 2_624,
        source_type: SourceType::Pronom,
        name: "602 Graph/Chart File",
        extensions: &["gc6"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x36, 0x30, 0x32, 0x0D,
                            0x0A, 0x43, 0x61, 0x6C, 0x63, 0x36, 0x30, 0x32, 0x20, 0x76, 0x2E,
                        ]),
                        Token::WildcardCountRange(1, 4),
                        Token::Literal(&[0x20, 0x47, 0x72, 0x61, 0x66]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_535,
        }],
    },
};
