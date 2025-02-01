use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_974: FileFormat = FileFormat {
    id: 1_779,
    puid: "fmt/974",
    name: "Notation Interchange File Format",
    extensions: &["nif"],
    media_types: &["application/vnd.music-niff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x4E, 0x49, 0x46, 0x46]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
