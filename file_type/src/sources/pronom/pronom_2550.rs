use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2550: FileType = FileType {
    file_format: &FileFormat {
        id: 2_550,
        source_type: SourceType::Pronom,
        name: "CATIA Model File",
        extensions: &["model"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x20, 0x20]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F,
                            0x4E, 0x20, 0x33,
                        ]),
                        Token::WildcardCount(11),
                        Token::Literal(&[0x52, 0x45, 0x4C, 0x45, 0x41, 0x53, 0x45, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
