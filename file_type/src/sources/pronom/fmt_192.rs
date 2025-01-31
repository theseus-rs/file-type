use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_192: FileFormat = FileFormat {
    id: 917,
    puid: "fmt/192",
    name: "Kodak Digital Camera Raw Image File",
    extensions: &["dcr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                    Token::WildcardCountRange(4, 16_384),
                    Token::Literal(&[0xFD, 0xE8]),
                    Token::WildcardCountRange(4, 16_384),
                    Token::Literal(&[0x4B, 0x6F, 0x64, 0x61, 0x6B]),
                    Token::WildcardCountRange(4, 256),
                    Token::Literal(&[0xFD, 0xE8]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_099,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
