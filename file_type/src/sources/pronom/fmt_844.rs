use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_844: FileFormat = FileFormat {
    id: 1_645,
    puid: "fmt/844",
    name: "Advanced Forensic Format",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x46, 0x46, 0x31, 0x30, 0x0D, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
