use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_623: FileFormat = FileFormat {
    id: 1_419,
    puid: "fmt/623",
    name: "SmartDraw",
    extensions: &["sdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4D, 0x41, 0x52, 0x54, 0x44, 0x52, 0x57, 0x01, 0x80,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
