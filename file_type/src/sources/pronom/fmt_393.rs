use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_393: FileFormat = FileFormat {
    id: 1_141,
    puid: "fmt/393",
    name: "Borland Reflex flat datafile",
    extensions: &["rxd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x02, 0x33, 0x51, 0x2E, 0x21, 0x26, 0x40, 0x23, 0x24, 0x21, 0x26, 0x26,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
