use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1409: FileFormat = FileFormat {
    id: 2_227,
    puid: "fmt/1409",
    name: "Flow Charting",
    extensions: &["fc5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0xFF, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
