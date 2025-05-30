use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1910: FileType = FileType {
    file_format: &FileFormat {
        id: 1_910,
        source_type: SourceType::Pronom,
        name: "Uuencoded File",
        extensions: &["uue"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x62, 0x65, 0x67, 0x69, 0x6E, 0x20]),
                            Token::Range(&[0x30], &[0x37]),
                            Token::Range(&[0x30], &[0x37]),
                            Token::Range(&[0x30], &[0x37]),
                            Token::Literal(&[0x20]),
                            Token::WildcardCountRange(1, 256),
                            Token::Literal(&[0x0A]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x0A, 0x65, 0x6E, 0x64])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
