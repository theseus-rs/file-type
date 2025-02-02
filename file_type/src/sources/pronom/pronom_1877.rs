use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1877: FileFormat = FileFormat {
    id: 1_877,
    source_type: SourceType::Pronom,
    name: "Preferred Executable Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x6F, 0x79, 0x21, 0x70, 0x65, 0x66, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
