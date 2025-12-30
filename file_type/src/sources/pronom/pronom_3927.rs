use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3927: FileType = FileType {
    file_format: &FileFormat {
        id: 3_927,
        source_type: SourceType::Pronom,
        name: "Cineon",
        extensions: &["cin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x2A, 0x5F, 0xD7])],
                },
            }],
        }],
        related_formats: &[],
    },
};
