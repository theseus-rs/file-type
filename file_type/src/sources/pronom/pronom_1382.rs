use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1382: FileType = FileType {
    file_format: &FileFormat {
        id: 1_382,
        source_type: SourceType::Pronom,
        name: "JPEG Extended Range",
        extensions: &["wdp", "jxr"],
        media_types: &["image/jxr"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0xBC, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
