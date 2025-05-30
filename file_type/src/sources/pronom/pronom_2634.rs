use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2634: FileType = FileType {
    file_format: &FileFormat {
        id: 2_634,
        source_type: SourceType::Pronom,
        name: "Animatic Film Format",
        extensions: &["flm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(48),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x27, 0x18, 0x28, 0x18])],
                },
            }],
        }],
        related_formats: &[],
    },
};
