use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1810: FileFormat = FileFormat {
    id: 2_661,
    puid: "fmt/1810",
    name: "Raw PIMA SWIR Reflectance Spectral File",
    extensions: &["fos"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x50, 0x4C, 0x46, 0x4F, 0x53, 0x00, 0x01, 0x05, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
