use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2809: FileType = FileType {
    file_format: &FileFormat {
        id: 2_809,
        source_type: SourceType::Pronom,
        name: "OpenWayback CDXJ File Format",
        extensions: &["cdx", "cdxj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x4F, 0x70, 0x65, 0x6E, 0x57, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6B,
                        0x2D, 0x43, 0x44, 0x58, 0x4A, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
