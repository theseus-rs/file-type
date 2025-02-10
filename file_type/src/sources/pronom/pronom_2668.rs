use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2668: FileType = FileType {
    file_format: &FileFormat {
        id: 2_668,
        source_type: SourceType::Pronom,
        name: "Direct Stream Digital Stream File",
        extensions: &["dsf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x44, 0x53, 0x44, 0x20]),
                        Token::WildcardCount(24),
                        Token::Literal(&[0x66, 0x6D, 0x74, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
