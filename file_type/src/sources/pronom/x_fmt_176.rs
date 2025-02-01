use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_176: FileFormat = FileFormat {
    id: 249,
    puid: "x-fmt/176",
    name: "Picture Publisher Bitmap",
    extensions: &["pp4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x49, 0x02, 0x01, 0x01, 0x00, 0x26, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
