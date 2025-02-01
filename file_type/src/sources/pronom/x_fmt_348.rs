use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_348: FileFormat = FileFormat {
    id: 512,
    puid: "x-fmt/348",
    name: "Multipage Zsoft Paintbrush Bitmap Graphics",
    extensions: &["dcx"],
    media_types: &["image/x-dcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB1, 0x68, 0xDE, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
