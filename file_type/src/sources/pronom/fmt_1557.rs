use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1557: FileFormat = FileFormat {
    id: 2_382,
    puid: "fmt/1557",
    name: "Cyber Paint Sequence",
    extensions: &["seq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xDB, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
