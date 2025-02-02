use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1666: FileFormat = FileFormat {
    id: 1_666,
    source_type: SourceType::Pronom,
    name: "Maya ASCII File Format",
    extensions: &["ma"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x4D, 0x61, 0x79, 0x61, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
