use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2478: FileFormat = FileFormat {
    id: 2_478,
    source_type: SourceType::Pronom,
    name: "Garmin Flexible and Interoperable Data Transfer File",
    extensions: &["fit"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
