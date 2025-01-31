use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1955: FileFormat = FileFormat {
    id: 2_820,
    puid: "fmt/1955",
    name: "Graphisoft Archicad Project",
    extensions: &["pla", "pln"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x6D, 0x6D]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x57, 0x57]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x41, 0x72, 0x63, 0x68, 0x69, 0x43, 0x41, 0x44]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
