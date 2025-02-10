use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1639: FileType = FileType {
    file_format: &FileFormat {
        id: 1_639,
        source_type: SourceType::Pronom,
        name: "Outlook Express Message Database",
        extensions: &["dbx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCF, 0xAD, 0x12, 0xFE, 0xC5, 0xFD, 0x74, 0x6F, 0x66, 0xE3, 0xD1, 0x11,
                        0x9A, 0x4E, 0x00, 0xC0, 0x4F, 0xA3, 0x09, 0xD4, 0x05, 0x00, 0x00, 0x00,
                        0x05, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
