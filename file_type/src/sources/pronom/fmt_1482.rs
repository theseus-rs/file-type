use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1482: FileFormat = FileFormat {
    id: 2_305,
    puid: "fmt/1482",
    name: "Access Report Snapshot",
    extensions: &["snp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                    Token::WildcardCount(56),
                    Token::Literal(&[
                        0x5F, 0x41, 0x63, 0x63, 0x52, 0x70, 0x74, 0x5F, 0x2E, 0x73, 0x6E, 0x70,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 801,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
