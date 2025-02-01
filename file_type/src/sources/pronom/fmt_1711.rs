use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1711: FileFormat = FileFormat {
    id: 2_547,
    puid: "fmt/1711",
    name: "Software602 Printer Configuration File",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x43, 0x36, 0x30, 0x32, 0x2D, 0x76, 0x2E, 0x31, 0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
