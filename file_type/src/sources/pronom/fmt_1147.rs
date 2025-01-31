use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1147: FileFormat = FileFormat {
    id: 1_958,
    puid: "fmt/1147",
    name: "Maxwell Render Scene File Format",
    extensions: &["mxs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF9, 0xB2, 0x2A, 0xD1]),
                    Token::WildcardCountRange(4, 32),
                    Token::Literal(&[0x6D, 0x78, 0x73, 0x74]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
