use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_209: FileFormat = FileFormat {
    id: 209,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite DCA Text File",
    extensions: &["dca"],
    media_types: &[],
    signatures: &[Signature {
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
