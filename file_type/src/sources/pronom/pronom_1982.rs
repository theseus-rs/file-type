use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1982: FileType = FileType {
    file_format: &FileFormat {
        id: 1_982,
        source_type: SourceType::Pronom,
        name: "Web Open Font Format",
        extensions: &["woff2"],
        media_types: &["font/woff2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x4F, 0x46, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
