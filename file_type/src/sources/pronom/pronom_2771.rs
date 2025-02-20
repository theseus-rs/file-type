use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2771: FileType = FileType {
    file_format: &FileFormat {
        id: 2_771,
        source_type: SourceType::Pronom,
        name: "Graphisoft Archicad Project",
        extensions: &["pln", "pla"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x4F, 0x46, 0x20, 0x46, 0x44, 0x42, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
