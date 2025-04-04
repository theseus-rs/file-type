use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1533: FileType = FileType {
    file_format: &FileFormat {
        id: 1_533,
        source_type: SourceType::Pronom,
        name: "SuperScape Virtual Reality Format",
        extensions: &["svr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x56, 0x52, 0x08, 0x00, 0x00, 0x00, 0x01, 0xEC,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
