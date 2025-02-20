use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1297: FileType = FileType {
    file_format: &FileFormat {
        id: 1_297,
        source_type: SourceType::Pronom,
        name: "PowerProject Teamplan",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(43),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x56, 0x65, 0x72,
                            0x73, 0x69, 0x6F, 0x6E,
                        ]),
                        Token::WildcardCount(15),
                        Token::Literal(&[0x61, 0xEA]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_298,
        }],
    },
};
