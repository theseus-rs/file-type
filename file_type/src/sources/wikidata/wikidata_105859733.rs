use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859733: FileFormat = FileFormat {
    id: 105_859_733,
    source_type: SourceType::Wikidata,
    name: "Brute Force and Ignorance video",
    extensions: &["bfi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x46, 0x26, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
