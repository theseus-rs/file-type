use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2154: FileType = FileType {
    file_format: &FileFormat {
        id: 2_154,
        source_type: SourceType::Pronom,
        name: "LEADTools Lead 1Bit Compressed Image",
        extensions: &["cmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x54, 0x52, 0x49, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
