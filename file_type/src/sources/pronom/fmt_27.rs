use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_27: FileFormat = FileFormat {
    id: 700,
    puid: "fmt/27",
    name: "AutoCAD Drawing",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x43]),
                    Token::Any(&[
                        &[Token::Literal(&[0x32, 0x2E, 0x32, 0x31])],
                        &[Token::Literal(&[0x32, 0x2E, 0x32, 0x32])],
                        &[Token::Literal(&[0x31, 0x30, 0x30, 0x31])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 716,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 701,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 699,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
