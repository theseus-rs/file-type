use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1583: FileFormat = FileFormat {
    id: 2_408,
    puid: "fmt/1583",
    name: "SXG (ZX Spectrum) Graphic File",
    extensions: &["sxg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x53, 0x58, 0x47, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
