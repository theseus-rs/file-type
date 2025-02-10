use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_67: FileType = FileType {
    file_format: &FileFormat {
        id: 67,
        source_type: SourceType::Pronom,
        name: "CorelDraw Compressed Drawing",
        extensions: &["cpx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
