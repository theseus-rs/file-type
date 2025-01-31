use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1178: FileFormat = FileFormat {
    id: 1_988,
    puid: "fmt/1178",
    name: "Synthetic Music Mobile Application Format",
    extensions: &["mmf"],
    media_types: &["application/vnd.yamaha.smaf-audio"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x4D, 0x4D, 0x44]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x43, 0x4E, 0x54, 0x49]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
