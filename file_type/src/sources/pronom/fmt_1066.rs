use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1066: FileFormat = FileFormat {
    id: 1_873,
    puid: "fmt/1066",
    name: "Portable Database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x3C, 0x3C, 0x50, 0x44, 0x42, 0x3A, 0x33, 0x3E, 0x3E, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_872,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
