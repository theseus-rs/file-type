use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1146: FileFormat = FileFormat {
    id: 1_956,
    puid: "fmt/1146",
    name: "Maxwell Render Image Format",
    extensions: &["mxi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(2),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x58, 0x49, 0x2E, 0x20, 0x4D, 0x61, 0x78, 0x77, 0x65, 0x6C, 0x6C, 0x20,
                    0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
