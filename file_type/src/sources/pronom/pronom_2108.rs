use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2108: FileFormat = FileFormat {
    id: 2_108,
    source_type: SourceType::Pronom,
    name: "RFFlow Chart",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x4C, 0x4F, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
