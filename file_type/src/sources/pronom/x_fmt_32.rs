use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_32: FileFormat = FileFormat {
    id: 63,
    puid: "x-fmt/32",
    name: "Harvard Graphics Chart",
    extensions: &["ch3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
