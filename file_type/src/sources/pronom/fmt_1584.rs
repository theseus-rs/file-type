use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1584: FileFormat = FileFormat {
    id: 2_409,
    puid: "fmt/1584",
    name: "ADRIFT Text Adventure File",
    extensions: &["taf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x42, 0x3F, 0xC9, 0x6A, 0x87, 0xC2, 0xCF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
