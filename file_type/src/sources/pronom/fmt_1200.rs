use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1200: FileFormat = FileFormat {
    id: 2_010,
    puid: "fmt/1200",
    name: "PowerDraw",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x64, 0x25, 0x34, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
