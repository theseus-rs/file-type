use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1610: FileFormat = FileFormat {
    id: 2_437,
    puid: "fmt/1610",
    name: "Viacom New Media Graphics",
    extensions: &["vnm", "000"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x4E, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
