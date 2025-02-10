use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2394: FileType = FileType {
    file_format: &FileFormat {
        id: 2_394,
        source_type: SourceType::Pronom,
        name: "Bitstream Speedo Fonts",
        extensions: &["spd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(100),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x31, 0x39,
                        0x38, 0x39, 0x2D, 0x31, 0x39, 0x39, 0x31, 0x20, 0x62, 0x79, 0x20, 0x42,
                        0x69, 0x74, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x20, 0x49, 0x6E, 0x63,
                        0x2E, 0x20, 0x41, 0x6C, 0x6C, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x73,
                        0x20, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
