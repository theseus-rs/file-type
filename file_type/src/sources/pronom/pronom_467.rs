use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_467: FileFormat = FileFormat {
    id: 467,
    source_type: SourceType::Pronom,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5C, 0x31, 0x63, 0x77, 0x20, 0x33, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
