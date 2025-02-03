use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_514: FileFormat = FileFormat {
    id: 514,
    source_type: SourceType::Pronom,
    name: "OmniPage Pro Document",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x4D, 0x45, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
