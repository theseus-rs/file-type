use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2810: FileFormat = FileFormat {
    id: 2_810,
    source_type: SourceType::Pronom,
    name: "Common Data Format dotCDF",
    extensions: &["cdf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_812,
    }],
};
