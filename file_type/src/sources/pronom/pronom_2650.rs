use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2650: FileFormat = FileFormat {
    id: 2_650,
    source_type: SourceType::Pronom,
    name: "FLExText Interlinear XML Format",
    extensions: &["flextext"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x3F, 0x3E,
                    ]),
                    Token::WildcardCountRange(0, 3),
                    Token::Literal(&[0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74]),
                    Token::WildcardCountRange(1, 256),
                    Token::Literal(&[
                        0x3C, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x6C, 0x69, 0x6E, 0x65, 0x61, 0x72,
                        0x2D, 0x74, 0x65, 0x78, 0x74,
                    ]),
                    Token::WildcardCountRange(1, 1_024),
                    Token::Literal(&[
                        0x3C, 0x70, 0x61, 0x72, 0x61, 0x67, 0x72, 0x61, 0x70, 0x68, 0x73,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
