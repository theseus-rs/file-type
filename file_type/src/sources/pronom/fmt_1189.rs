use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1189: FileFormat = FileFormat {
    id: 1_999,
    puid: "fmt/1189",
    name: "Ogre Mesh XML",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x6D, 0x65, 0x73, 0x68, 0x3E]),
                        Token::WildcardCountRange(0, 64),
                        Token::Literal(&[
                            0x3C, 0x73, 0x75, 0x62, 0x6D, 0x65, 0x73, 0x68, 0x65, 0x73, 0x3E,
                        ]),
                        Token::WildcardCountRange(0, 8),
                        Token::Literal(&[0x3C, 0x73, 0x75, 0x62, 0x6D, 0x65, 0x73, 0x68]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x2F, 0x6D, 0x65, 0x73, 0x68, 0x3E])],
                },
            },
        ],
    }],
    related_formats: &[],
};
