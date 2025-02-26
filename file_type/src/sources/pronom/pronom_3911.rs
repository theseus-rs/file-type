use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3911: FileType = FileType {
    file_format: &FileFormat {
        id: 3_911,
        source_type: SourceType::Pronom,
        name: "Plextalk Project File (imdn)",
        extensions: &["imdn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x01, 0xFF, 0x00, 0xFF]),
                        Token::WildcardCount(32),
                        Token::Literal(&[
                            0x49, 0x6D, 0x64, 0x54, 0x78, 0x74, 0x54, 0x61, 0x62, 0x6C, 0x2E, 0x69,
                            0x6D, 0x74, 0x74,
                        ]),
                        Token::WildcardCount(17),
                        Token::Literal(&[
                            0x49, 0x6D, 0x64, 0x50, 0x68, 0x72, 0x49, 0x6E, 0x66, 0x6F, 0x2E, 0x69,
                            0x6D, 0x70, 0x68,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
