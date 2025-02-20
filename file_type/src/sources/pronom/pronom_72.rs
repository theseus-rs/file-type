use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_72: FileType = FileType {
    file_format: &FileFormat {
        id: 72,
        source_type: SourceType::Pronom,
        name: "Data Interchange Format",
        extensions: &["dif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x54, 0x41, 0x42, 0x4C, 0x45, 0x0D, 0x0A, 0x30, 0x2C, 0x31, 0x0D, 0x0A,
                            0x22,
                        ]),
                        Token::WildcardCountRange(1, 100),
                        Token::Literal(&[
                            0x22, 0x0D, 0x0A, 0x56, 0x45, 0x43, 0x54, 0x4F, 0x52, 0x53, 0x0D, 0x0A,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
