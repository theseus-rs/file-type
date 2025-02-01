use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_357: FileFormat = FileFormat {
    id: 1_103,
    puid: "fmt/357",
    name: "3GPP Audio/Video File",
    extensions: &["3gp", "3gpp"],
    media_types: &["audio/3gpp", "video/3gpp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x33, 0x67]),
                    Token::Any(&[
                        &[Token::Literal(&[0x65])],
                        &[Token::Literal(&[0x67])],
                        &[Token::Literal(&[0x68])],
                        &[Token::Literal(&[0x70])],
                        &[Token::Literal(&[0x72])],
                        &[Token::Literal(&[0x73])],
                        &[Token::Literal(&[0x74])],
                    ]),
                    Token::Range(&[0x34], &[0x39]),
                    Token::Literal(&[0x00, 0x00]),
                    Token::Range(&[0x00], &[0x03]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_102,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 924,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
