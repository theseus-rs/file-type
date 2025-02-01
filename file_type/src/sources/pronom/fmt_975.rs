use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_975: FileFormat = FileFormat {
    id: 1_780,
    puid: "fmt/975",
    name: "Jamcracker Tracker Module",
    extensions: &["jam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x45, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
