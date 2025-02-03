use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2749: FileFormat = FileFormat {
    id: 2_749,
    source_type: SourceType::Pronom,
    name: "Microsoft Agent File",
    extensions: &["acs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC3, 0xAB, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
