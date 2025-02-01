use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_315: FileFormat = FileFormat {
    id: 1_060,
    puid: "fmt/315",
    name: "Play SID Audio",
    extensions: &["sid", "psid"],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x53, 0x49, 0x44, 0x00]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::Literal(&[0x00, 0x7C]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_059,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
