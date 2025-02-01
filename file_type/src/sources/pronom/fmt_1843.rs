use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1843: FileFormat = FileFormat {
    id: 2_695,
    puid: "fmt/1843",
    name: "Human Machine Interfaces HMI File",
    extensions: &["hmi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4D, 0x49, 0x2D, 0x4D, 0x49, 0x44, 0x49, 0x53, 0x4F, 0x4E, 0x47, 0x30,
                    0x36, 0x31, 0x35, 0x39, 0x35,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
