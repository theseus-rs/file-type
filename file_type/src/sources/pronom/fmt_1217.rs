use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1217: FileFormat = FileFormat {
    id: 2_027,
    puid: "fmt/1217",
    name: "Leonardo Image Format",
    extensions: &["leo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x65, 0x6F, 0x20, 0x7A, 0x6F, 0x6E, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D,
                    0x61, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
