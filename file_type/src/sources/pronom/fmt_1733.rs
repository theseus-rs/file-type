use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1733: FileFormat = FileFormat {
    id: 2_578,
    puid: "fmt/1733",
    name: "PaintShop Plus Compressed Format",
    extensions: &["psc", "da4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x6D, 0x38, 0x39, 0x50, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
