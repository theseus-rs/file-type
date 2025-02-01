use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_722: FileFormat = FileFormat {
    id: 1_521,
    puid: "fmt/722",
    name: "Oktalyzer Audio file",
    extensions: &["okt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x4B, 0x54, 0x41, 0x53, 0x4F, 0x4E, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
