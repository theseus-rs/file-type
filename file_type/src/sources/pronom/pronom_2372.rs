use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2372: FileType = FileType {
    file_format: &FileFormat {
        id: 2_372,
        source_type: SourceType::Pronom,
        name: "Daisy-Dot Font File",
        extensions: &["nlq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x9B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
