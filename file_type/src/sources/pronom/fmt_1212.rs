use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1212: FileFormat = FileFormat {
    id: 2_022,
    puid: "fmt/1212",
    name: "HP System Software Manager CVA File",
    extensions: &["cva"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x56, 0x41, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x49, 0x6E, 0x66,
                    0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
