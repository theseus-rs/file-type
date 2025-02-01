use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1504: FileFormat = FileFormat {
    id: 2_327,
    puid: "fmt/1504",
    name: "Agisoft Tiled Model",
    extensions: &["tls"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4C, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
