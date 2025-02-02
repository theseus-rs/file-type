use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1531: FileFormat = FileFormat {
    id: 1_531,
    source_type: SourceType::Pronom,
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_530,
    }],
};
