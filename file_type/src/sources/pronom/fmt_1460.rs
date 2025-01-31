use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1460: FileFormat = FileFormat {
    id: 2_283,
    puid: "fmt/1460",
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
                    Token::Literal(&[0x72, 0x4C, 0x61, 0x75, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
