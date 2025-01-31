use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1591: FileFormat = FileFormat {
    id: 2_418,
    puid: "fmt/1591",
    name: "ESRI ArcInfo Coverage Annotation File",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x27, 0x0A, 0xFF, 0xFF, 0xFF, 0xBD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
