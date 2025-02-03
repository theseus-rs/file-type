use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2450: FileFormat = FileFormat {
    id: 2_450,
    source_type: SourceType::Pronom,
    name: "Art Of Noise",
    extensions: &["aon"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
