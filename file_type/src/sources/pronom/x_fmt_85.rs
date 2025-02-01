use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_85: FileFormat = FileFormat {
    id: 130,
    puid: "x-fmt/85",
    name: "Picture Publisher Bitmap",
    extensions: &["pp5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x50, 0x55, 0x42, 0x49, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
