use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_510: FileFormat = FileFormat {
    id: 1_297,
    puid: "fmt/510",
    name: "PowerProject Teamplan",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 1_298,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
