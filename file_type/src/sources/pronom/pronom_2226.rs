use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2226: FileFormat = FileFormat {
    id: 2_226,
    source_type: SourceType::Pronom,
    name: "Flow Charting",
    extensions: &["gfc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0C, 0xEF, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
