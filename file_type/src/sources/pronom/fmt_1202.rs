use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1202: FileFormat = FileFormat {
    id: 2_012,
    puid: "fmt/1202",
    name: "Guymager Acquisition Info File",
    extensions: &["info"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x55, 0x59, 0x4D, 0x41, 0x47, 0x45, 0x52, 0x20, 0x41, 0x43, 0x51, 0x55,
                    0x49, 0x53, 0x49, 0x54, 0x49, 0x4F, 0x4E, 0x20, 0x49, 0x4E, 0x46, 0x4F, 0x20,
                    0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
