use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_325: FileFormat = FileFormat {
    id: 488,
    puid: "x-fmt/325",
    name: "Harvard Graphics Vector Graphics",
    extensions: &["cht"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x61, 0x6C, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
