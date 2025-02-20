use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2293: FileType = FileType {
    file_format: &FileFormat {
        id: 2_293,
        source_type: SourceType::Pronom,
        name: "MIG Graphics File",
        extensions: &["mig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x58, 0x4D, 0x49, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
