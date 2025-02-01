use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1003: FileFormat = FileFormat {
    id: 1_808,
    puid: "fmt/1003",
    name: "Nearly Raw Raster Data",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x52, 0x52, 0x44, 0x30, 0x30, 0x30, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_809,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_807,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
