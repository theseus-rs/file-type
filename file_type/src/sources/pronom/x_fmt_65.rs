use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_65: FileFormat = FileFormat {
    id: 106,
    puid: "x-fmt/65",
    name: "Microsoft Word for Macintosh Document",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x37, 0x00, 0x23])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 8,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 105,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 1_091,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
