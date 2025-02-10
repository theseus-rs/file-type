use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2765: FileType = FileType {
    file_format: &FileFormat {
        id: 2_765,
        source_type: SourceType::Pronom,
        name: "TibetDoc Word Document",
        extensions: &["dct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x4B, 0x54, 0x43]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x54, 0x69, 0x62, 0x65, 0x74, 0x44, 0x6F, 0x63]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
