use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1092: FileFormat = FileFormat {
    id: 1_900,
    puid: "fmt/1092",
    name: "Alias Pix Image File",
    extensions: &["pix", "ico"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x18]),
                    Token::NotLiteral(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 802,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
