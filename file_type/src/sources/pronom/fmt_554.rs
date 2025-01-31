use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_554: FileFormat = FileFormat {
    id: 1_342,
    puid: "fmt/554",
    name: "Microsoft Excel Chart",
    extensions: &["xlc"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x02, 0x06, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x20, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 679,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
