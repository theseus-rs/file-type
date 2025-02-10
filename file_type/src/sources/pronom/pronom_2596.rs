use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2596: FileType = FileType {
    file_format: &FileFormat {
        id: 2_596,
        source_type: SourceType::Pronom,
        name: "Canon CIF File",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x49, 0x01, 0x00, 0x11, 0x14, 0x11, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
