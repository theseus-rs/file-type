use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1518: FileType = FileType {
    file_format: &FileFormat {
        id: 1_518,
        source_type: SourceType::Pronom,
        name: "MultiTracker Module",
        extensions: &["mtm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x54, 0x4D, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
