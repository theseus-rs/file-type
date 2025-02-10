use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2745: FileType = FileType {
    file_format: &FileFormat {
        id: 2_745,
        source_type: SourceType::Pronom,
        name: "Open Access III Document",
        extensions: &["ext"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x03, 0x4C, 0x54, 0x32, 0x6A, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
