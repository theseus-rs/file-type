use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1088: FileType = FileType {
    file_format: &FileFormat {
        id: 1_088,
        source_type: SourceType::Pronom,
        name: "Microsoft Project Export File",
        extensions: &["mpx"],
        media_types: &["application/x-project"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x50, 0x58]),
                        Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                        Token::WildcardCountRange(1, 50),
                        Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                        Token::Literal(&[0x33, 0x2E, 0x30]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 324,
        }],
    },
};
