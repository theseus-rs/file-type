use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1952: FileFormat = FileFormat {
    id: 2_816,
    puid: "fmt/1952",
    name: "PechaMaker Format",
    extensions: &["pxp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x70, 0x78, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
