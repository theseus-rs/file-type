use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1104: FileType = FileType {
    file_format: &FileFormat {
        id: 1_104,
        source_type: SourceType::Pronom,
        name: "Internet Data Query File",
        extensions: &["idq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B, 0x51, 0x75, 0x65, 0x72, 0x79, 0x5D]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x63])]]),
                            Token::Literal(&[0x69]),
                            Token::Any(&[&[Token::Literal(&[0x53])], &[Token::Literal(&[0x73])]]),
                            Token::Literal(&[0x63, 0x6F, 0x70, 0x65, 0x3D]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B, 0x51, 0x75, 0x65, 0x72, 0x79, 0x5D]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x63])]]),
                            Token::Literal(&[0x69]),
                            Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x63])]]),
                            Token::Literal(&[0x6F, 0x6C, 0x75, 0x6D, 0x6E, 0x73, 0x3D]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B, 0x51, 0x75, 0x65, 0x72, 0x79, 0x5D]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x63])]]),
                            Token::Literal(&[0x69]),
                            Token::Any(&[&[Token::Literal(&[0x54])], &[Token::Literal(&[0x74])]]),
                            Token::Literal(&[0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x3D, 0x2F]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x5B, 0x51, 0x75, 0x65, 0x72, 0x79, 0x5D]),
                            Token::AnyWildcard,
                            Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x63])]]),
                            Token::Literal(&[0x69]),
                            Token::Any(&[&[Token::Literal(&[0x52])], &[Token::Literal(&[0x72])]]),
                            Token::Literal(&[
                                0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x3D,
                            ]),
                            Token::WildcardCountRange(0, 1),
                            Token::Literal(&[0x25]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
