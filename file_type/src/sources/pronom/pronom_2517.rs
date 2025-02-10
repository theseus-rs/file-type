use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2517: FileType = FileType {
    file_format: &FileFormat {
        id: 2_517,
        source_type: SourceType::Pronom,
        name: "OBO Flat File Format",
        extensions: &["obo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
