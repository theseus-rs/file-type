use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2414: FileType = FileType {
    file_format: &FileFormat {
        id: 2_414,
        source_type: SourceType::Pronom,
        name: "Taquart Interlace Picture",
        extensions: &["tip"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x50, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
