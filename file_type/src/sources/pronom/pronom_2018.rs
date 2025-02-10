use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2018: FileType = FileType {
    file_format: &FileFormat {
        id: 2_018,
        source_type: SourceType::Pronom,
        name: "Virtools File Format",
        extensions: &["cmo", "nmo", "vmo", "nms"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x6D, 0x6F, 0x20, 0x46, 0x69, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
