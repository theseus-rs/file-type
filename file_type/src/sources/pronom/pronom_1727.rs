use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1727: FileType = FileType {
    file_format: &FileFormat {
        id: 1_727,
        source_type: SourceType::Pronom,
        name: "Xar Image Format",
        extensions: &["xar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x41, 0x52, 0x41, 0xA3, 0xA3, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
