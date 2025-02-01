use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_316: FileFormat = FileFormat {
    id: 1_061,
    puid: "fmt/316",
    name: "Real SID Audio",
    extensions: &["sid"],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x53, 0x49, 0x44, 0x00, 0x02, 0x00, 0x7C]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
