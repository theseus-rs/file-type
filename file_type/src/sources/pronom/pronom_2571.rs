use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2571: FileType = FileType {
    file_format: &FileFormat {
        id: 2_571,
        source_type: SourceType::Pronom,
        name: "Pro Tools Session File",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x03, 0x30, 0x30, 0x31, 0x30, 0x31, 0x31, 0x31, 0x31, 0x30, 0x30, 0x31,
                            0x30, 0x31, 0x30, 0x31, 0x31,
                        ]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x5A, 0x01, 0x00, 0x04]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
