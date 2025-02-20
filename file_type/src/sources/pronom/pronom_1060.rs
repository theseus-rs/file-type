use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1060: FileType = FileType {
    file_format: &FileFormat {
        id: 1_060,
        source_type: SourceType::Pronom,
        name: "Play SID Audio",
        extensions: &["sid", "psid"],
        media_types: &["audio/prs.sid"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x53, 0x49, 0x44, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                        Token::Literal(&[0x00, 0x7C]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_059,
        }],
    },
};
