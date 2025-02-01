use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_300: FileFormat = FileFormat {
    id: 1_044,
    puid: "fmt/300",
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x43, 0x48, 0x49, 0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x20, 0x34, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
