use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2294: FileFormat = FileFormat {
    id: 2_294,
    source_type: SourceType::Pronom,
    name: "Multi Palette Picture File",
    extensions: &["mpp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
