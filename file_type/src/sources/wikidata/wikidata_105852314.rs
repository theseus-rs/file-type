use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852314: FileFormat = FileFormat {
    id: 105_852_314,
    source_type: SourceType::Wikidata,
    name: "Snzip compressed (framing format)",
    extensions: &["sz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x06, 0x00, 0x73, 0x4E, 0x61, 0x50, 0x70, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
