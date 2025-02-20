use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1333: FileType = FileType {
    file_format: &FileFormat {
        id: 1_333,
        source_type: SourceType::Pronom,
        name: "Macromedia FreeHand",
        extensions: &["fh8"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
