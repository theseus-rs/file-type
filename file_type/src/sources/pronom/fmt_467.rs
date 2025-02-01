use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_467: FileFormat = FileFormat {
    id: 1_254,
    puid: "fmt/467",
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x4C, 0x65, 0x00, 0x45, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(77),
                    Token::Literal(&[0x80, 0x40, 0x01, 0x00, 0x00, 0x80, 0x40]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
