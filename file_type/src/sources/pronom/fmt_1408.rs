use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1408: FileFormat = FileFormat {
    id: 2_226,
    puid: "fmt/1408",
    name: "Flow Charting",
    extensions: &["gfc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0C, 0xEF, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
