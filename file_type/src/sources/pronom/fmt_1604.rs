use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1604: FileFormat = FileFormat {
    id: 2_431,
    puid: "fmt/1604",
    name: "EggPaint (Atari Falcon)",
    extensions: &["trp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x52, 0x55, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
