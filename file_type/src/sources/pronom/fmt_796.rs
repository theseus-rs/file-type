use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_796: FileFormat = FileFormat {
    id: 1_595,
    puid: "fmt/796",
    name: "Adobe After Effects",
    extensions: &["aep"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x45, 0x67, 0x67, 0x21]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
