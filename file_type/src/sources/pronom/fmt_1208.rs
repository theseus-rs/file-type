use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1208: FileFormat = FileFormat {
    id: 2_018,
    puid: "fmt/1208",
    name: "Virtools File Format",
    extensions: &["cmo", "nmo", "vmo", "nms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x6D, 0x6F, 0x20, 0x46, 0x69, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
