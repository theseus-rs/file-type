use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2641: FileFormat = FileFormat {
    id: 2_641,
    source_type: SourceType::Pronom,
    name: "Haiku Vector Icon Format",
    extensions: &["hvif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x63, 0x69, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
