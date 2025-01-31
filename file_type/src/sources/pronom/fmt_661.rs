use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_661: FileFormat = FileFormat {
    id: 1_460,
    puid: "fmt/661",
    name: "Sigma RAW Image",
    extensions: &["x3f"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4F, 0x56, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
