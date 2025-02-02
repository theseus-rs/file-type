use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2595: FileFormat = FileFormat {
    id: 2_595,
    source_type: SourceType::Pronom,
    name: "Canon MIF File",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x49, 0x01, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
