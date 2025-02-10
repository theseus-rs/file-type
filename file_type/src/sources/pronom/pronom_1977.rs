use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1977: FileType = FileType {
    file_format: &FileFormat {
        id: 1_977,
        source_type: SourceType::Pronom,
        name: "Softimage 3D Picture File Format",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x53, 0x80, 0xF6, 0x34]),
                        Token::WildcardCount(84),
                        Token::Literal(&[0x50, 0x49, 0x43, 0x54]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
