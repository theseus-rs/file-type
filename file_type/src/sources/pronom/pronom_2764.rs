use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2764: FileType = FileType {
    file_format: &FileFormat {
        id: 2_764,
        source_type: SourceType::Pronom,
        name: "Jupiter Tesselation (JT) File",
        extensions: &["jt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                        Token::WildcardCountRange(5, 6),
                        Token::Literal(&[0x4A, 0x54]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
