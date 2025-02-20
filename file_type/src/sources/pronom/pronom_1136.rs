use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1136: FileType = FileType {
    file_format: &FileFormat {
        id: 1_136,
        source_type: SourceType::Pronom,
        name: "Internet Calendar and Scheduling format",
        extensions: &["ics"],
        media_types: &["text/calendar"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x43, 0x41, 0x4C, 0x45,
                                0x4E, 0x44, 0x41, 0x52,
                            ]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E]),
                            Token::WildcardCountRange(0, 2),
                            Token::Literal(&[0x3A, 0x32, 0x2E, 0x30]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x45, 0x4E, 0x44, 0x3A, 0x56, 0x43, 0x41, 0x4C, 0x45, 0x4E, 0x44, 0x41,
                            0x52,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
