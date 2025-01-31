use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_878: FileFormat = FileFormat {
    id: 1_682,
    puid: "fmt/878",
    name: "Corel Presentation",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x0F, 0x0F, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
