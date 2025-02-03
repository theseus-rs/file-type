use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2161: FileFormat = FileFormat {
    id: 2_161,
    source_type: SourceType::Pronom,
    name: "PTGui Project File",
    extensions: &["pts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x70, 0x74, 0x47, 0x75, 0x69, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65,
                    0x63, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
