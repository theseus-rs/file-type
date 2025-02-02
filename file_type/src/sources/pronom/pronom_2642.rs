use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2642: FileFormat = FileFormat {
    id: 2_642,
    source_type: SourceType::Pronom,
    name: "ICDRAW Single Icon File",
    extensions: &["ibi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
