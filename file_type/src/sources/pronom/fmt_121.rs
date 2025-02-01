use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_121: FileFormat = FileFormat {
    id: 768,
    puid: "fmt/121",
    name: "DROID Signature File Format",
    extensions: &["xml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::WildcardCountRange(0, 50),
                    Token::Literal(&[
                        0x3C, 0x46, 0x46, 0x53, 0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65,
                        0x46, 0x69, 0x6C, 0x65, 0x20,
                    ]),
                    Token::WildcardCountRange(0, 100),
                    Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
