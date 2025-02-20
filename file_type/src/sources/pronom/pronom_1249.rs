use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1249: FileType = FileType {
    file_format: &FileFormat {
        id: 1_249,
        source_type: SourceType::Pronom,
        name: "MS-DOS Compression Format (SZDD Variant)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x5A, 0x44, 0x44, 0x88, 0xF0, 0x27, 0x33, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
