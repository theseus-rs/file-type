use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1309: FileFormat = FileFormat {
    id: 1_309,
    source_type: SourceType::Pronom,
    name: "Open Project File",
    extensions: &["pod"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6D, 0x2E, 0x70, 0x72, 0x6F, 0x6A, 0x69, 0x74, 0x79, 0x2E, 0x73,
                    0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x64, 0x61, 0x74, 0x61, 0x2E, 0x50, 0x72,
                    0x6F, 0x6A, 0x65, 0x63, 0x74, 0x44, 0x61, 0x74, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
