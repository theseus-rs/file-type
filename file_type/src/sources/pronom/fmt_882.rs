use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_882: FileFormat = FileFormat {
    id: 1_686,
    puid: "fmt/882",
    name: "Wordstar 2000",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7F, 0x20, 0x57, 0x53, 0x32, 0x30, 0x30, 0x30, 0xFF, 0x31, 0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 379,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
