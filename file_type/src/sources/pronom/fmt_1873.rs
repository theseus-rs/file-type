use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1873: FileFormat = FileFormat {
    id: 2_727,
    puid: "fmt/1873",
    name: "Guitar Pro File",
    extensions: &["gp3", "gp4", "gp5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x46, 0x49, 0x43, 0x48, 0x49, 0x45, 0x52, 0x20, 0x47, 0x55, 0x49, 0x54,
                    0x41, 0x52, 0x20, 0x50, 0x52, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
