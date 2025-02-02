use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2147: FileFormat = FileFormat {
    id: 2_147,
    source_type: SourceType::Pronom,
    name: "Avery Label Pro Document",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCC, 0xAA, 0x03, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
