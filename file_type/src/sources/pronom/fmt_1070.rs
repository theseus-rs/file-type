use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1070: FileFormat = FileFormat {
    id: 1_877,
    puid: "fmt/1070",
    name: "Preferred Executable Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x6F, 0x79, 0x21, 0x70, 0x65, 0x66, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
