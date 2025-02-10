use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2576: FileType = FileType {
    file_format: &FileFormat {
        id: 2_576,
        source_type: SourceType::Pronom,
        name: "PowerGraphics Image File",
        extensions: &["pgr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x77, 0x65, 0x72, 0x47, 0x46, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
