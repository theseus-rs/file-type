use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_843: FileType = FileType {
    file_format: &FileFormat {
        id: 843,
        source_type: SourceType::Pronom,
        name: "CATIA Model",
        extensions: &["mod", "model"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(80),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x20, 0x20]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x53, 0x4F, 0x4C, 0x55, 0x54, 0x49,
                            0x4F, 0x4E, 0x53, 0x20, 0x56, 0x34,
                        ]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x52, 0x45, 0x4C, 0x45, 0x41, 0x53, 0x45, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
