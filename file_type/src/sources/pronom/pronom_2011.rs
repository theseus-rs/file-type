use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2011: FileFormat = FileFormat {
    id: 2_011,
    source_type: SourceType::Pronom,
    name: "PowerCADD",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3F, 0x3F, 0x3F, 0x3F, 0x02, 0x57, 0x02, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
