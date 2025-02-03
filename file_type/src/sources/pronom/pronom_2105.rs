use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2105: FileFormat = FileFormat {
    id: 2_105,
    source_type: SourceType::Pronom,
    name: "Envoy Document File",
    extensions: &["evy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
