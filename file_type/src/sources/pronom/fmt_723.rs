use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_723: FileFormat = FileFormat {
    id: 1_522,
    puid: "fmt/723",
    name: "Farandole Composer Module",
    extensions: &["far"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x41, 0x52, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
