use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1410: FileFormat = FileFormat {
    id: 2_228,
    puid: "fmt/1410",
    name: "Flow Charting",
    extensions: &["fcx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x36, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
