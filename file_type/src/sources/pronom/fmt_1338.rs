use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1338: FileFormat = FileFormat {
    id: 2_156,
    puid: "fmt/1338",
    name: "RootsMagic Database",
    extensions: &["rmgc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x33, 0x00,
                    ]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[
                        0x46, 0x61, 0x6D, 0x69, 0x6C, 0x79, 0x54, 0x61, 0x62, 0x6C, 0x65,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_528,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
