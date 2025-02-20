use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2037: FileType = FileType {
    file_format: &FileFormat {
        id: 2_037,
        source_type: SourceType::Pronom,
        name: "NMRView",
        extensions: &["nv"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x34, 0x18, 0xAB, 0xCD])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xCD, 0xAB, 0x18, 0x34])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
