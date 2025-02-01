use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1157: FileFormat = FileFormat {
    id: 1_967,
    puid: "fmt/1157",
    name: "Folio Infobase File",
    extensions: &["nfo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1_292),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x6F, 0x6C, 0x69, 0x6F])],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1_292),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x32, 0x30, 0x30])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
