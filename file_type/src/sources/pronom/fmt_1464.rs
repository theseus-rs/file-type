use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1464: FileFormat = FileFormat {
    id: 2_287,
    puid: "fmt/1464",
    name: "Maestro Music File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x65, 0x73, 0x74, 0x72, 0x6F, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
