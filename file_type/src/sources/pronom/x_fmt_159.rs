use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_159: FileFormat = FileFormat {
    id: 223,
    puid: "x-fmt/159",
    name: "GEM Image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x01, 0x00, 0x08, 0x00]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x04])]]),
                    Token::Literal(&[0x00, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_484,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
