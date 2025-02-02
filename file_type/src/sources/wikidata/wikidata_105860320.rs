use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860320: FileFormat = FileFormat {
    id: 105_860_320,
    source_type: SourceType::Wikidata,
    name: "Fugawi route format",
    extensions: &["rte"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x46, 0x55, 0x47, 0x52, 0x54,
                    0x45, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
