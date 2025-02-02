use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862612: FileFormat = FileFormat {
    id: 105_862_612,
    source_type: SourceType::Wikidata,
    name: "Statistica MFM data",
    extensions: &["mfm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x53, 0x53, 0x34, 0x4C, 0xD9, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
