use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_324: FileFormat = FileFormat {
    id: 487,
    puid: "x-fmt/324",
    name: "Harvard Graphics Show",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
