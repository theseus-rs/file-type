use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2306: FileFormat = FileFormat {
    id: 2_306,
    source_type: SourceType::Pronom,
    name: "Mar Archive",
    extensions: &["mar", "mac"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
