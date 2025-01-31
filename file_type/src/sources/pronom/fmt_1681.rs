use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1681: FileFormat = FileFormat {
    id: 2_517,
    puid: "fmt/1681",
    name: "OBO Flat File Format",
    extensions: &["obo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
