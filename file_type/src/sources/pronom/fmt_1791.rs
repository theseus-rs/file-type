use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1791: FileFormat = FileFormat {
    id: 2_641,
    puid: "fmt/1791",
    name: "Haiku Vector Icon Format",
    extensions: &["hvif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x63, 0x69, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
