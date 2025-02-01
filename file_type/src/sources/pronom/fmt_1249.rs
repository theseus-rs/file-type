use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1249: FileFormat = FileFormat {
    id: 2_067,
    puid: "fmt/1249",
    name: "SOSI",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x48, 0x4F, 0x44, 0x45]),
                    Token::WildcardCountRange(0, 300),
                    Token::Literal(&[
                        0x53, 0x4F, 0x53, 0x49, 0x2D, 0x56, 0x45, 0x52, 0x53, 0x4A, 0x4F, 0x4E,
                    ]),
                    Token::WildcardCountRange(0, 5),
                    Token::Literal(&[0x34, 0x2E, 0x35]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_064,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
