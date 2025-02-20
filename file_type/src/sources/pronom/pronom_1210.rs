use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1210: FileType = FileType {
    file_format: &FileFormat {
        id: 1_210,
        source_type: SourceType::Pronom,
        name: "Harris Matrix",
        extensions: &["hm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00,
                            0x10, 0x00, 0x43, 0x48, 0x61, 0x72, 0x72, 0x69, 0x73, 0x4D, 0x61, 0x74,
                            0x72, 0x69, 0x78, 0x44, 0x6F, 0x63,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x02, 0x26, 0x6E, 0x00]),
                            Token::WildcardCount(1),
                            Token::Literal(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
