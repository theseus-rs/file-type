use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2570: FileType = FileType {
    file_format: &FileFormat {
        id: 2_570,
        source_type: SourceType::Pronom,
        name: "Geosoft Map Description File",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(56),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x20, 0x78, 0x73, 0x2C, 0x79, 0x73, 0x2C, 0x6D, 0x62, 0x2C, 0x6D,
                        0x72, 0x2C, 0x6D, 0x74, 0x2C, 0x6D, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
