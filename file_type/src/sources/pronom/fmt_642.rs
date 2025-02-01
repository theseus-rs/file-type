use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_642: FileFormat = FileFormat {
    id: 1_441,
    puid: "fmt/642",
    name: "Fujifilm RAW Image Format",
    extensions: &["raf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x4A, 0x49, 0x46, 0x49, 0x4C, 0x4D, 0x43, 0x43, 0x44, 0x2D, 0x52,
                    0x41, 0x57, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
