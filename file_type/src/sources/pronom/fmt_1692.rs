use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1692: FileFormat = FileFormat {
    id: 2_528,
    puid: "fmt/1692",
    name: "ESRI ArcGIS Raw Raster Reader/ Writer",
    extensions: &["hdr"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x23, 0x20, 0x41, 0x52, 0x43, 0x2F, 0x49, 0x4E, 0x46, 0x4F, 0x20, 0x66,
                            0x69, 0x6C, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20,
                            0x66, 0x72, 0x6F, 0x6D, 0x20, 0x45, 0x52, 0x20, 0x4D, 0x61, 0x70, 0x70,
                            0x65, 0x72,
                        ]),
                        Token::WildcardCountRange(4, 257),
                        Token::Literal(&[0x62, 0x79, 0x74, 0x65, 0x6F, 0x72, 0x64, 0x65, 0x72]),
                        Token::WildcardCountRange(3, 4),
                        Token::Literal(&[0x6E, 0x62, 0x69, 0x74, 0x73]),
                        Token::WildcardCountRange(3, 4),
                        Token::Literal(&[0x78, 0x64, 0x69, 0x6D]),
                        Token::WildcardCountRange(1, 15),
                        Token::Literal(&[0x79, 0x64, 0x69, 0x6D]),
                        Token::WildcardCountRange(1, 15),
                        Token::Literal(&[0x6E, 0x63, 0x6F, 0x6C, 0x73]),
                        Token::WildcardCountRange(2, 6),
                        Token::Literal(&[0x6E, 0x72, 0x6F, 0x77, 0x73]),
                        Token::WildcardCountRange(4, 6),
                        Token::Literal(&[0x6E, 0x62, 0x61, 0x6E, 0x64, 0x73]),
                        Token::WildcardCountRange(3, 4),
                        Token::Literal(&[0x75, 0x6C, 0x78, 0x6D, 0x61, 0x70]),
                        Token::WildcardCountRange(9, 15),
                        Token::Literal(&[0x75, 0x6C, 0x79, 0x6D, 0x61, 0x70]),
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
                        Token::Literal(&[0x42, 0x59, 0x54, 0x45, 0x4F, 0x52, 0x44, 0x45, 0x52]),
                        Token::WildcardCount(7),
                        Token::Literal(&[0x0A, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54]),
                        Token::WildcardCount(9),
                        Token::Literal(&[0x42, 0x49, 0x4C, 0x0A, 0x4E, 0x52, 0x4F, 0x57, 0x53]),
                        Token::WildcardCountRange(13, 15),
                        Token::Literal(&[0x0A, 0x4E, 0x43, 0x4F, 0x4C, 0x53]),
                        Token::WildcardCountRange(13, 16),
                        Token::Literal(&[0x0A, 0x4E, 0x42, 0x41, 0x4E, 0x44, 0x53]),
                        Token::WildcardCount(10),
                        Token::Literal(&[0x0A, 0x4E, 0x42, 0x49, 0x54, 0x53]),
                        Token::WildcardCount(12),
                        Token::Literal(&[
                            0x0A, 0x42, 0x41, 0x4E, 0x44, 0x52, 0x4F, 0x57, 0x42, 0x59, 0x54, 0x45,
                            0x53,
                        ]),
                        Token::WildcardCountRange(7, 8),
                        Token::Literal(&[
                            0x0A, 0x54, 0x4F, 0x54, 0x41, 0x4C, 0x52, 0x4F, 0x57, 0x42, 0x59, 0x54,
                            0x45, 0x53,
                        ]),
                        Token::WildcardCountRange(6, 7),
                        Token::Literal(&[
                            0x0A, 0x50, 0x49, 0x58, 0x45, 0x4C, 0x54, 0x59, 0x50, 0x45,
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
