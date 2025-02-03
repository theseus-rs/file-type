use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855921: FileFormat = FileFormat {
    id: 105_855_921,
    source_type: SourceType::Wikidata,
    name: "Altera Design Library System list",
    extensions: &["dls"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4C, 0x53, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
