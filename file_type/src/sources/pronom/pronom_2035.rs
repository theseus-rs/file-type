use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2035: FileType = FileType {
    file_format: &FileFormat {
        id: 2_035,
        source_type: SourceType::Pronom,
        name: "PaperPort MAX",
        extensions: &["max"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x45, 0x6D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
