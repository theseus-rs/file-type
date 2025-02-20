use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1515: FileType = FileType {
    file_format: &FileFormat {
        id: 1_515,
        source_type: SourceType::Pronom,
        name: "MOD Audio Module",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1_080),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x2E, 0x4B, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
