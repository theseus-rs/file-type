use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2407: FileFormat = FileFormat {
    id: 2_407,
    source_type: SourceType::Pronom,
    name: "Vim SWAP File",
    extensions: &["swp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x30, 0x56, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
