use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_590: FileFormat = FileFormat {
    id: 1_382,
    puid: "fmt/590",
    name: "JPEG Extended Range",
    extensions: &["wdp", "jxr"],
    media_types: &["image/jxr"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0xBC, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
