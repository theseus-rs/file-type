use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1906: FileType = FileType {
    file_format: &FileFormat {
        id: 1_906,
        source_type: SourceType::Pronom,
        name: "TCR eBook",
        extensions: &["tcr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x21, 0x38, 0x2D, 0x42, 0x69, 0x74, 0x21, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
