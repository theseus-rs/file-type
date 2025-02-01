use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_148: FileFormat = FileFormat {
    id: 209,
    puid: "x-fmt/148",
    name: "IBM DisplayWrite DCA Text File",
    extensions: &["dca"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x05, 0xE1, 0x03, 0x00, 0x00, 0x20, 0xE2, 0x05, 0x00, 0x01, 0x51, 0x01,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
