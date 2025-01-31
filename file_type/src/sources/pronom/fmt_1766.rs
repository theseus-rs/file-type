use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1766: FileFormat = FileFormat {
    id: 2_616,
    puid: "fmt/1766",
    name: "Sony SML File",
    extensions: &["sml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x6C, 0x69, 0x73, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x3D, 0x22,
                        ]),
                        Token::WildcardCountRange(0, 64),
                        Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x3D]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x2F, 0x6C, 0x69, 0x73, 0x74, 0x3E])],
                },
            },
        ],
    }],
    related_formats: &[],
};
