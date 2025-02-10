use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2411: FileType = FileType {
    file_format: &FileFormat {
        id: 2_411,
        source_type: SourceType::Pronom,
        name: "TheDraw Save File",
        extensions: &["td"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x15, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x53, 0x61, 0x76,
                        0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
