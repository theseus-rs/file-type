use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_716: FileFormat = FileFormat {
    id: 1_515,
    puid: "fmt/716",
    name: "MOD Audio Module",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1_080),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x2E, 0x4B, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
