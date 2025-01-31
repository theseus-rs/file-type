use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_799: FileFormat = FileFormat {
    id: 1_599,
    puid: "fmt/799",
    name: "WriteNow",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x72, 0x69, 0x74, 0x65, 0x4E, 0x6F, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
