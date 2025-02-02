use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853413: FileFormat = FileFormat {
    id: 105_853_413,
    source_type: SourceType::Wikidata,
    name: "Doom Eternal Resource Archives",
    extensions: &["streamdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0xA5, 0xC2, 0x29, 0x2E, 0xF3, 0xC7, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
