use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2283: FileType = FileType {
    file_format: &FileFormat {
        id: 2_283,
        source_type: SourceType::Pronom,
        name: "Stuffit Archive File",
        extensions: &["sit"],
        media_types: &["application/x-stuffit"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x53, 0x49, 0x54, 0x21]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x72, 0x4C, 0x61, 0x75, 0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
