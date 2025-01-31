use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_912: FileFormat = FileFormat {
    id: 1_717,
    puid: "fmt/912",
    name: "Microsoft Paint",
    extensions: &["msp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4C, 0x69, 0x6E, 0x53]),
                    Token::WildcardCount(22),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 302,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
