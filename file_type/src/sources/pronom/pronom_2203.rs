use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2203: FileType = FileType {
    file_format: &FileFormat {
        id: 2_203,
        source_type: SourceType::Pronom,
        name: "Bruker PDZ",
        extensions: &["pdz", "xpdz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x01, 0x17, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
