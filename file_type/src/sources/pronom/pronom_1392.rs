use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1392: FileFormat = FileFormat {
    id: 1_392,
    source_type: SourceType::Pronom,
    name: "eXtensible ARchive format",
    extensions: &["xar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x61, 0x72, 0x21, 0x00, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
