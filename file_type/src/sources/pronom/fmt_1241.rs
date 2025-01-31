use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1241: FileFormat = FileFormat {
    id: 2_059,
    puid: "fmt/1241",
    name: "FO File",
    extensions: &["fo"],
    media_types: &["application/vnd.software602.filler.form+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x6F, 0x3A, 0x72, 0x6F, 0x6F, 0x74])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
