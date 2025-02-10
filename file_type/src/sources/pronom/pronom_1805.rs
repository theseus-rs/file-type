use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1805: FileType = FileType {
    file_format: &FileFormat {
        id: 1_805,
        source_type: SourceType::Pronom,
        name: "TZX Format",
        extensions: &["tzx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x54, 0x61, 0x70, 0x65, 0x21, 0x1A, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
