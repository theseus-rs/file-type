use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_961: FileFormat = FileFormat {
    id: 1_766,
    puid: "fmt/961",
    name: "Mobile eXtensible Music Format",
    extensions: &["mxmf"],
    media_types: &["audio/mobile-xmf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x58, 0x4D, 0x46, 0x5F, 0x32, 0x2E, 0x30, 0x30, 0x00, 0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_513,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
