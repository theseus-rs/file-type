use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2054: FileType = FileType {
    file_format: &FileFormat {
        id: 2_054,
        source_type: SourceType::Pronom,
        name: "EclipseCrossword Word List File",
        extensions: &["ewl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x20, 0x45, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x43, 0x72, 0x6F,
                        0x73, 0x73, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x77, 0x6F, 0x72, 0x64, 0x20,
                        0x6C, 0x69, 0x73, 0x74, 0x0D, 0x0A, 0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
