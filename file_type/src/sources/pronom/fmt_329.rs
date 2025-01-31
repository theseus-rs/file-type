use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_329: FileFormat = FileFormat {
    id: 1_074,
    puid: "fmt/329",
    name: "Shell Archive Format",
    extensions: &["shar"],
    media_types: &["application/x-sh", "application/x-shar"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x23, 0x21]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[
                            0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x73, 0x68, 0x0A, 0x23, 0x20, 0x54, 0x68,
                            0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x68, 0x65, 0x6C,
                            0x6C, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
                        0x73, 0x68, 0x65, 0x6C, 0x6C, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
