use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2688: FileType = FileType {
    file_format: &FileFormat {
        id: 2_688,
        source_type: SourceType::Pronom,
        name: "Brio Query File",
        extensions: &["bqy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x42, 0x52, 0x49, 0x46, 0x20, 0x42, 0x49, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
