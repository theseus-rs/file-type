use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2405: FileType = FileType {
    file_format: &FileFormat {
        id: 2_405,
        source_type: SourceType::Pronom,
        name: "Envision Publisher File",
        extensions: &["evp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6E, 0x56, 0x69, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x50, 0x75, 0x62,
                        0x6C, 0x69, 0x73, 0x68, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
