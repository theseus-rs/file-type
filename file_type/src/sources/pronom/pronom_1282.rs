use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1282: FileType = FileType {
    file_format: &FileFormat {
        id: 1_282,
        source_type: SourceType::Pronom,
        name: "ATCO-CIF",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x43, 0x4F, 0x2D, 0x43, 0x49, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
