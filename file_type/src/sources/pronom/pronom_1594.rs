use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1594: FileType = FileType {
    file_format: &FileFormat {
        id: 1_594,
        source_type: SourceType::Pronom,
        name: "RPM Package Manager file",
        extensions: &["rpm", "src.rpm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
