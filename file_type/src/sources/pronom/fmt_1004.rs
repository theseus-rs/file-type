use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1004: FileFormat = FileFormat {
    id: 1_809,
    puid: "fmt/1004",
    name: "Nearly Raw Raster Data",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x52, 0x52, 0x44, 0x30, 0x30, 0x30, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_810,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_808,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
