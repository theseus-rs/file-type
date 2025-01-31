use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_36: FileFormat = FileFormat {
    id: 67,
    puid: "x-fmt/36",
    name: "CorelDraw Compressed Drawing",
    extensions: &["cpx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
