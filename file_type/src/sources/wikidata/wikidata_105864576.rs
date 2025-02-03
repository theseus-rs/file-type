use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864576: FileFormat = FileFormat {
    id: 105_864_576,
    source_type: SourceType::Wikidata,
    name: "In-a-Vision drawing",
    extensions: &["pic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0xFF, 0x01, 0x04, 0x03, 0x06, 0x00, 0x01, 0x00, 0x04, 0x01, 0xFF, 0xFF,
                    0xFF, 0x00, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
