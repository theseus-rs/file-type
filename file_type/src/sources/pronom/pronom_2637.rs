use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2637: FileType = FileType {
    file_format: &FileFormat {
        id: 2_637,
        source_type: SourceType::Pronom,
        name: "G9B Graphics Format Bitmap",
        extensions: &["g9b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x39, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
