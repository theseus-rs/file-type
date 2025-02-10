use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1595: FileType = FileType {
    file_format: &FileFormat {
        id: 1_595,
        source_type: SourceType::Pronom,
        name: "Adobe After Effects",
        extensions: &["aep"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x45, 0x67, 0x67, 0x21]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
