use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_457: FileType = FileType {
    file_format: &FileFormat {
        id: 457,
        source_type: SourceType::Pronom,
        name: "Aldus Freehand Drawing",
        extensions: &["fh3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x48, 0x33, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
