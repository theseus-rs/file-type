use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1750: FileFormat = FileFormat {
    id: 2_596,
    puid: "fmt/1750",
    name: "Canon CIF File",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x49, 0x01, 0x00, 0x11, 0x14, 0x11, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
