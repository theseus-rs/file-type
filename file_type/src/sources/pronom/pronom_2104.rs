use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2104: FileType = FileType {
    file_format: &FileFormat {
        id: 2_104,
        source_type: SourceType::Pronom,
        name: "Envoy Document File",
        extensions: &["evy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
