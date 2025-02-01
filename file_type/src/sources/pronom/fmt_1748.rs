use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1748: FileFormat = FileFormat {
    id: 2_594,
    puid: "fmt/1748",
    name: "Microsoft PowerPoint Presentation",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xED, 0xDE, 0xAD, 0x0B, 0x03, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 133,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_593,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
