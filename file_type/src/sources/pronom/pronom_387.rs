use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_387: FileType = FileType {
    file_format: &FileFormat {
        id: 387,
        source_type: SourceType::Pronom,
        name: "BZIP Compressed Archive",
        extensions: &["bz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x5A, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
