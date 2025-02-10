use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1334: FileType = FileType {
    file_format: &FileFormat {
        id: 1_334,
        source_type: SourceType::Pronom,
        name: "Macromedia FreeHand",
        extensions: &["fh9"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
