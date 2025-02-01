use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_823: FileFormat = FileFormat {
    id: 1_623,
    puid: "fmt/823",
    name: "P00 C64 Image Format",
    extensions: &["p00", "p01", "p02", "p03", "p04"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x36, 0x34, 0x46, 0x69, 0x6C, 0x65, 0x00]),
                    Token::WildcardCount(16),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
