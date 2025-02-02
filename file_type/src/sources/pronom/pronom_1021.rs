use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1021: FileFormat = FileFormat {
    id: 1_021,
    source_type: SourceType::Pronom,
    name: "LaTeX (Subdocument)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x5C]),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x75, 0x73, 0x65, 0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x7B,
                        ])],
                        &[Token::Literal(&[
                            0x63, 0x68, 0x61, 0x70, 0x74, 0x65, 0x72, 0x7B,
                        ])],
                        &[Token::Literal(&[
                            0x73, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x7B,
                        ])],
                        &[Token::Literal(&[
                            0x73, 0x75, 0x62, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x7B,
                        ])],
                        &[Token::Literal(&[0x62, 0x65, 0x67, 0x69, 0x6E, 0x7B])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_020,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
    ],
};
