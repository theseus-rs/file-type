use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1893: FileFormat = FileFormat {
    id: 1_893,
    source_type: SourceType::Pronom,
    name: "TRIM Context Reference File",
    extensions: &["tr5", "txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x52, 0x49, 0x4D, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x78, 0x74,
                    0x20, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
