use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1586: FileFormat = FileFormat {
    id: 2_411,
    puid: "fmt/1586",
    name: "TheDraw Save File",
    extensions: &["td"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x15, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x53, 0x61, 0x76, 0x65,
                    0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
