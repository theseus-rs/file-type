use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858048: FileFormat = FileFormat {
    id: 105_858_048,
    source_type: SourceType::Wikidata,
    name: "InstaCalc spreadsheet (v3)",
    extensions: &["ins"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x0B, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x01, 0xBA, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
