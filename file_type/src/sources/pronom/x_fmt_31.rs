use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_31: FileFormat = FileFormat {
    id: 62,
    puid: "x-fmt/31",
    name: "CorelDraw Compressed Drawing",
    extensions: &["cdx", "cjw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
