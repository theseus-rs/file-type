use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1717: FileFormat = FileFormat {
    id: 1_717,
    source_type: SourceType::Pronom,
    name: "Microsoft Paint",
    extensions: &["msp"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 302,
    }],
};
