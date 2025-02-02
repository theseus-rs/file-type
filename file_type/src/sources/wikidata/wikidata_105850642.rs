use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850642: FileFormat = FileFormat {
    id: 105_850_642,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Chart (v2.x)",
    extensions: &["cht"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x61, 0x6C, 0x63, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
