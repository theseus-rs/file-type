use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2293: FileFormat = FileFormat {
    id: 2_293,
    source_type: SourceType::Pronom,
    name: "MIG Graphics File",
    extensions: &["mig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x58, 0x4D, 0x49, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
