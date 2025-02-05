use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2327: FileFormat = FileFormat {
    id: 2_327,
    source_type: SourceType::Pronom,
    name: "Agisoft Tiled Model",
    extensions: &["tls"],
    media_types: &[],
    signatures: &[Signature {
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
