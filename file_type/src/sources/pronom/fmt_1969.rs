use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1969: FileFormat = FileFormat {
    id: 2_835,
    puid: "fmt/1969",
    name: "ETC Express/Expression Show File",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x54, 0x43, 0x20, 0x45, 0x58, 0x50, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
