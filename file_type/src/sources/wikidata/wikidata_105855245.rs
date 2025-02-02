use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855245: FileFormat = FileFormat {
    id: 105_855_245,
    source_type: SourceType::Wikidata,
    name: "FIDAP Neutral format",
    extensions: &["fdneut"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x20, 0x46, 0x49, 0x44, 0x41, 0x50, 0x20, 0x4E, 0x45, 0x55, 0x54,
                    0x52, 0x41, 0x4C, 0x20, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
