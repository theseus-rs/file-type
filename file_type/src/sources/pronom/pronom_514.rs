use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_514: FileType = FileType {
    file_format: &FileFormat {
        id: 514,
        source_type: SourceType::Pronom,
        name: "OmniPage Pro Document",
        extensions: &["met"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x43, 0x4D, 0x45, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
