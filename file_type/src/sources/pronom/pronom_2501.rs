use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2501: FileFormat = FileFormat {
    id: 2_501,
    source_type: SourceType::Pronom,
    name: "Easy CD Creator Layout | Roxio Easy CD Creator Layout",
    extensions: &["rcl", "cl5"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x10, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(1),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_502,
    }],
};
