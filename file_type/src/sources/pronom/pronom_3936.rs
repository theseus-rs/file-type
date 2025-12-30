use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3936: FileType = FileType {
    file_format: &FileFormat {
        id: 3_936,
        source_type: SourceType::Pronom,
        name: "MOOF Disk Image File",
        extensions: &["moof"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x4F, 0x46, 0xFF, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
