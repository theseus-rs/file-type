use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1679: FileFormat = FileFormat {
    id: 2_515,
    puid: "fmt/1679",
    name: "Garmin track log file",
    extensions: &["gmn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x40, 0x67, 0x41, 0x72, 0x4D, 0x69, 0x4E, 0x40, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
