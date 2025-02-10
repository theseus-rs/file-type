use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2836: FileType = FileType {
    file_format: &FileFormat {
        id: 2_836,
        source_type: SourceType::Pronom,
        name: "MOXCEL",
        extensions: &["mxl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x58, 0x43, 0x45, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
