use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2157: FileType = FileType {
    file_format: &FileFormat {
        id: 2_157,
        source_type: SourceType::Pronom,
        name: "PaperPort MAX",
        extensions: &["max"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x46, 0x6B, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
