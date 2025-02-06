use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1686: FileFormat = FileFormat {
    id: 1_686,
    source_type: SourceType::Pronom,
    name: "Wordstar 2000",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 379,
    }],
};
