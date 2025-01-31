use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1622: FileFormat = FileFormat {
    id: 2_449,
    puid: "fmt/1622",
    name: "Asylum Music Format",
    extensions: &["amf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x53, 0x59, 0x4C, 0x55, 0x4D, 0x20, 0x4D, 0x75, 0x73, 0x69, 0x63, 0x20,
                    0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
