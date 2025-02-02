use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2675: FileFormat = FileFormat {
    id: 2_675,
    source_type: SourceType::Pronom,
    name: "Audacity Project File",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x31, 0x2E, 0x30]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCountRange(0, 1_024),
                    Token::Literal(&[
                        0x61, 0x75, 0x64, 0x61, 0x63, 0x69, 0x74, 0x79, 0x76, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
