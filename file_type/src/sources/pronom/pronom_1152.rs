use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1152: FileType = FileType {
    file_format: &FileFormat {
        id: 1_152,
        source_type: SourceType::Pronom,
        name: "RealAudio",
        extensions: &["ra"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2E, 0x72, 0x61, 0xFD, 0x00, 0x04, 0x00, 0x00, 0x2E, 0x72, 0x61, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
