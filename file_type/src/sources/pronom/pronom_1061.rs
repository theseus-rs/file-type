use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1061: FileType = FileType {
    file_format: &FileFormat {
        id: 1_061,
        source_type: SourceType::Pronom,
        name: "Real SID Audio",
        extensions: &["sid"],
        media_types: &["audio/prs.sid"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x53, 0x49, 0x44, 0x00, 0x02, 0x00, 0x7C]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
