use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_214: FileFormat = FileFormat {
    id: 302,
    puid: "x-fmt/214",
    name: "Microsoft Paint",
    extensions: &["msp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x61, 0x6E, 0x4D]),
                    Token::WildcardCount(22),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_717,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
