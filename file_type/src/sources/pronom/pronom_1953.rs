use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1953: FileType = FileType {
    file_format: &FileFormat {
        id: 1_953,
        source_type: SourceType::Pronom,
        name: "ZISRAW (CZI) File Format",
        extensions: &["czi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x49, 0x53, 0x52, 0x41, 0x57, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
