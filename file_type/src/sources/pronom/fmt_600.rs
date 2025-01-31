use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_600: FileFormat = FileFormat {
    id: 1_392,
    puid: "fmt/600",
    name: "eXtensible ARchive format",
    extensions: &["xar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x61, 0x72, 0x21, 0x00, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
