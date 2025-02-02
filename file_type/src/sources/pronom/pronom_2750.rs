use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2750: FileFormat = FileFormat {
    id: 2_750,
    source_type: SourceType::Pronom,
    name: "RagTime Document File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x68, 0x8F, 0x68, 0x8F, 0x68, 0x8F, 0xFF, 0xF3,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
