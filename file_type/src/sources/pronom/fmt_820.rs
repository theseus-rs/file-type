use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_820: FileFormat = FileFormat {
    id: 1_620,
    puid: "fmt/820",
    name: "T64 Tape Image Format",
    extensions: &["t64"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x36, 0x34, 0x53, 0x20, 0x74, 0x61, 0x70, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
