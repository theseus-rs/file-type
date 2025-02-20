use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1525: FileType = FileType {
    file_format: &FileFormat {
        id: 1_525,
        source_type: SourceType::Pronom,
        name: "Virtual Disk Image",
        extensions: &["vdi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(64),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x10, 0xDA, 0xBE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
