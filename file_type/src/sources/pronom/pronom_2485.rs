use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2485: FileType = FileType {
    file_format: &FileFormat {
        id: 2_485,
        source_type: SourceType::Pronom,
        name: "XL-Paint MaX",
        extensions: &["max", "xlp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4C, 0x50, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
