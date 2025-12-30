use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3937: FileType = FileType {
    file_format: &FileFormat {
        id: 3_937,
        source_type: SourceType::Pronom,
        name: "AV1 Image File Format",
        extensions: &["avif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x74, 0x79, 0x70, 0x61, 0x76, 0x69, 0x66,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
