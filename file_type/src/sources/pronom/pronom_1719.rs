use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1719: FileType = FileType {
    file_format: &FileFormat {
        id: 1_719,
        source_type: SourceType::Pronom,
        name: "Caligari trueSpace File Format",
        extensions: &["cob", "scn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x6C, 0x69, 0x67, 0x61, 0x72, 0x69, 0x20, 0x56, 0x30, 0x30,
                        0x2E, 0x30, 0x31, 0x42,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
