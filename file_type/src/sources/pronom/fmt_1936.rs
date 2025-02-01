use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1936: FileFormat = FileFormat {
    id: 2_797,
    puid: "fmt/1936",
    name: "PCRaster",
    extensions: &["csf", "map"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x55, 0x55, 0x20, 0x43, 0x52, 0x4F, 0x53, 0x53, 0x20, 0x53, 0x59, 0x53,
                    0x54, 0x45, 0x4D, 0x20, 0x4D, 0x41, 0x50, 0x20, 0x46, 0x4F, 0x52, 0x4D, 0x41,
                    0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
