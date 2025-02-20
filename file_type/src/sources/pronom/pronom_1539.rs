use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1539: FileType = FileType {
    file_format: &FileFormat {
        id: 1_539,
        source_type: SourceType::Pronom,
        name: "ClarisWorks Spreadsheet",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x04]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                        Token::WildcardCount(248),
                        Token::Literal(&[0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_544,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_536,
            },
        ],
    },
};
