use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_610: FileFormat = FileFormat {
    id: 1_406,
    puid: "fmt/610",
    name: "ARJ File Format",
    extensions: &["arj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60, 0xEA])],
            },
        }],
    }],
    related_formats: &[],
};
