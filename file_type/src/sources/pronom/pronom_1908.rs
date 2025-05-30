use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1908: FileType = FileType {
    file_format: &FileFormat {
        id: 1_908,
        source_type: SourceType::Pronom,
        name: "yEnc Encoded File",
        extensions: &["yenc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3D, 0x79, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x6C, 0x69, 0x6E, 0x65,
                        0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
