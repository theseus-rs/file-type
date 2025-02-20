use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1381: FileType = FileType {
    file_format: &FileFormat {
        id: 1_381,
        source_type: SourceType::Pronom,
        name: "Windows Media Playlist",
        extensions: &["wpl"],
        media_types: &["application/vnd.ms-wpl"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x77, 0x70, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
