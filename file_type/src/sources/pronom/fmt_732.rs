use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_732: FileFormat = FileFormat {
    id: 1_531,
    puid: "fmt/732",
    name: "Bink Video Format",
    extensions: &["bik2", "bk2"],
    media_types: &["video/vnd.radgamettools.bink"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4B, 0x42, 0x32]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61])],
                        &[Token::Literal(&[0x64])],
                        &[Token::Literal(&[0x66])],
                        &[Token::Literal(&[0x67])],
                        &[Token::Literal(&[0x68])],
                        &[Token::Literal(&[0x69])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_530,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
