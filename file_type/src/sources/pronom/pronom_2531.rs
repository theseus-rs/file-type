use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2531: FileType = FileType {
    file_format: &FileFormat {
        id: 2_531,
        source_type: SourceType::Pronom,
        name: "602 Text file",
        extensions: &["602"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x43, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
