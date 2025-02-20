use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1675: FileType = FileType {
    file_format: &FileFormat {
        id: 1_675,
        source_type: SourceType::Pronom,
        name: "Adobe Content Server Message File",
        extensions: &["acsm"],
        media_types: &["application/vnd.adobe.adept+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x66, 0x75, 0x6C, 0x66, 0x69, 0x6C, 0x6C, 0x6D, 0x65, 0x6E, 0x74,
                            0x54, 0x6F, 0x6B, 0x65, 0x6E, 0x20, 0x66, 0x75, 0x6C, 0x66, 0x69, 0x6C,
                            0x6C, 0x6D, 0x65, 0x6E, 0x74, 0x54, 0x79, 0x70, 0x65,
                        ]),
                        Token::WildcardCountRange(5, 64),
                        Token::Literal(&[
                            0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                            0x2F, 0x2F, 0x6E, 0x73, 0x2E, 0x61, 0x64, 0x6F, 0x62, 0x65, 0x2E, 0x63,
                            0x6F, 0x6D, 0x2F, 0x61, 0x64, 0x65, 0x70, 0x74, 0x22,
                        ]),
                        Token::WildcardCountRange(3, 64),
                        Token::Literal(&[
                            0x3C, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6F, 0x72,
                        ]),
                        Token::WildcardCountRange(3, 128),
                        Token::Literal(&[
                            0x3C, 0x6F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x55, 0x52, 0x4C,
                        ]),
                        Token::WildcardCountRange(3, 64),
                        Token::Literal(&[
                            0x3C, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                        ]),
                        Token::WildcardCountRange(3, 64),
                        Token::Literal(&[
                            0x3C, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        ]),
                        Token::WildcardCountRange(3, 512),
                        Token::Literal(&[0x3C, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61]),
                        Token::WildcardCountRange(16, 2_048),
                        Token::Literal(&[
                            0x3C, 0x6C, 0x69, 0x63, 0x65, 0x6E, 0x73, 0x65, 0x54, 0x6F, 0x6B, 0x65,
                            0x6E,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
