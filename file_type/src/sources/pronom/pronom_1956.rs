use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1956: FileType = FileType {
    file_format: &FileFormat {
        id: 1_956,
        source_type: SourceType::Pronom,
        name: "Maxwell Render Image Format",
        extensions: &["mxi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x58, 0x49, 0x2E, 0x20, 0x4D, 0x61, 0x78, 0x77, 0x65, 0x6C, 0x6C,
                        0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
