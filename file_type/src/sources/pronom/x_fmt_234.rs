use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_234: FileFormat = FileFormat {
    id: 326,
    puid: "x-fmt/234",
    name: "Paint Shop Pro Image",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x69, 0x6E, 0x74, 0x20, 0x53, 0x68, 0x6F, 0x70, 0x20, 0x50, 0x72,
                    0x6F, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A,
                    0x1A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 450,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 555,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
