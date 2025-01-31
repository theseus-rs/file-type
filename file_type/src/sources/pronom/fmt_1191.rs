use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1191: FileFormat = FileFormat {
    id: 2_001,
    puid: "fmt/1191",
    name: "Adobe InDesign Book",
    extensions: &["indb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x06, 0xED, 0xF5, 0xD8, 0x1D, 0x46, 0xE5, 0xBD, 0x31, 0xEF, 0xE7, 0xFE,
                    0x74, 0xB7, 0x1D, 0x42, 0x4F, 0x4F, 0x4B, 0x42, 0x4F, 0x4F, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
