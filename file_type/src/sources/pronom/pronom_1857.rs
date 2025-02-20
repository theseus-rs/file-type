use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1857: FileType = FileType {
    file_format: &FileFormat {
        id: 1_857,
        source_type: SourceType::Pronom,
        name: "BKNAS Seismic Data Format",
        extensions: &["bknas"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4B, 0x4E, 0x41, 0x53, 0x20, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
