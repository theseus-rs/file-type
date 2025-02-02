use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1198: FileFormat = FileFormat {
    id: 1_198,
    source_type: SourceType::Pronom,
    name: "Apple Core Audio Format",
    extensions: &["caf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x61, 0x66, 0x66, 0x00, 0x01, 0x00, 0x00, 0x64, 0x65, 0x73, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
