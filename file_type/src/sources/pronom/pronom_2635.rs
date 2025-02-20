use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2635: FileType = FileType {
    file_format: &FileFormat {
        id: 2_635,
        source_type: SourceType::Pronom,
        name: "FLR Database File",
        extensions: &["flr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x32, 0x32, 0x34, 0x39])],
                },
            }],
        }],
        related_formats: &[],
    },
};
