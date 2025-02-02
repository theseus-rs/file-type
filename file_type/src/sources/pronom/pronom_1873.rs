use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1873: FileFormat = FileFormat {
    id: 1_873,
    source_type: SourceType::Pronom,
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_872,
    }],
};
