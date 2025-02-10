use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1907: FileType = FileType {
    file_format: &FileFormat {
        id: 1_907,
        source_type: SourceType::Pronom,
        name: "XZ File Format",
        extensions: &["xz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x59, 0x5A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
