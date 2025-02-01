use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_821: FileFormat = FileFormat {
    id: 1_621,
    puid: "fmt/821",
    name: "G64 GCR-encoded Disk Image Format",
    extensions: &["g41", "g64", "g71"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x43, 0x52, 0x2D, 0x31, 0x35, 0x34, 0x31, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
