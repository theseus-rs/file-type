use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1343: FileFormat = FileFormat {
    id: 1_343,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Macro",
    extensions: &["xlm"],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 678,
    }],
};
