use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1056: FileFormat = FileFormat {
    id: 1_861,
    puid: "fmt/1056",
    name: "SNAP Main Data File",
    extensions: &["mdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x41, 0x50, 0x20, 0x4D, 0x61, 0x69, 0x6E, 0x20, 0x64, 0x61, 0x74,
                    0x61, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
