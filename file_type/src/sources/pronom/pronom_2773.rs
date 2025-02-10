use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2773: FileType = FileType {
    file_format: &FileFormat {
        id: 2_773,
        source_type: SourceType::Pronom,
        name: "ActiveMime Object",
        extensions: &["mso"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x4D, 0x69, 0x6D, 0x65,
                        ]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x01, 0x0F]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
