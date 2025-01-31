use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_555: FileFormat = FileFormat {
    id: 1_343,
    puid: "fmt/555",
    name: "Microsoft Excel Macro",
    extensions: &["xlm"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x00, 0x04, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x40, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 678,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
