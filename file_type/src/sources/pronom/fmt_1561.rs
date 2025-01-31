use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1561: FileFormat = FileFormat {
    id: 2_386,
    puid: "fmt/1561",
    name: "SpritePad Image Format",
    extensions: &["spd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
