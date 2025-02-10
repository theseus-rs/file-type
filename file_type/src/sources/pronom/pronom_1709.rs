use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1709: FileType = FileType {
    file_format: &FileFormat {
        id: 1_709,
        source_type: SourceType::Pronom,
        name: "Bluetooth Snoop Packet Capture",
        extensions: &["log"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x74, 0x73, 0x6E, 0x6F, 0x6F, 0x70, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
