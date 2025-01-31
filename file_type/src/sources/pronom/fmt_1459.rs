use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1459: FileFormat = FileFormat {
    id: 2_282,
    puid: "fmt/1459",
    name: "Stuffit Archive File",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53, 0x49, 0x54, 0x21]),
                    Token::WildcardCount(6),
                    Token::Literal(&[0x72, 0x4C, 0x61, 0x75, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
