use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2217: FileFormat = FileFormat {
    id: 2_217,
    source_type: SourceType::Pronom,
    name: "DiskDoubler",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAB, 0xCD, 0x00, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
