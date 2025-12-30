use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3935: FileType = FileType {
    file_format: &FileFormat {
        id: 3_935,
        source_type: SourceType::Pronom,
        name: "WOZ Disk Image File",
        extensions: &["woz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x4F, 0x5A, 0x32, 0xFF, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
