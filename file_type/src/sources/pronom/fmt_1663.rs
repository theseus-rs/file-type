use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1663: FileFormat = FileFormat {
    id: 2_499,
    puid: "fmt/1663",
    name: "YAODL (Yet Another Object Description Language) File",
    extensions: &["ydl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x59, 0x41, 0x4F, 0x44, 0x4C, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
