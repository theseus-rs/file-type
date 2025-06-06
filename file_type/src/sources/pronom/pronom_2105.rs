use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2105: FileType = FileType {
    file_format: &FileFormat {
        id: 2_105,
        source_type: SourceType::Pronom,
        name: "Envoy Document File",
        extensions: &["evy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
                },
            }],
        }],
        related_formats: &[],
    },
};
