use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1154: FileFormat = FileFormat {
    id: 1_964,
    puid: "fmt/1154",
    name: "Lightwright Show File",
    extensions: &["lw4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x57, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x20, 0x56, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
