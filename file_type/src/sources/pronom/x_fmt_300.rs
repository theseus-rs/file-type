use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_300: FileFormat = FileFormat {
    id: 453,
    puid: "x-fmt/300",
    name: "X-Windows Screen Dump File",
    extensions: &["xdm", "xwd"],
    media_types: &["image/x-xwindowdump"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x06]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
