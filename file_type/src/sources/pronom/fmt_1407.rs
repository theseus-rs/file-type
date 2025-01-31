use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1407: FileFormat = FileFormat {
    id: 2_225,
    puid: "fmt/1407",
    name: "Flow Charting",
    extensions: &["fcd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x4F, 0x43, 0x48, 0x54, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
