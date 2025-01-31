use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1498: FileFormat = FileFormat {
    id: 2_321,
    puid: "fmt/1498",
    name: "Cool Edit/Adobe Audition Session File",
    extensions: &["ses"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4F, 0x4C, 0x4E, 0x45, 0x53, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
