use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1537: FileFormat = FileFormat {
    id: 2_361,
    puid: "fmt/1537",
    name: "Serif PagePlus Publication",
    extensions: &["ppp", "ppt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCE, 0x19, 0xCE, 0x19])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_470,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
