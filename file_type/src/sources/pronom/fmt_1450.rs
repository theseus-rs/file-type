use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1450: FileFormat = FileFormat {
    id: 2_270,
    puid: "fmt/1450",
    name: "Aldus FreeHand Drawing",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x48, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
