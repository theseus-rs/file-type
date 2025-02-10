use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1052: FileType = FileType {
    file_format: &FileFormat {
        id: 1_052,
        source_type: SourceType::Pronom,
        name: "Quicken Interchange Format",
        extensions: &["qif"],
        media_types: &["application/qif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x21, 0x54, 0x79, 0x70, 0x65, 0x3A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x5E]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
