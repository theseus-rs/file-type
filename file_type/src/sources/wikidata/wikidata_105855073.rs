use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855073: FileFormat = FileFormat {
    id: 105_855_073,
    source_type: SourceType::Wikidata,
    name: "DACT compressed data",
    extensions: &["dct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x43, 0x54, 0xC3])],
            },
        }],
    }],
    related_formats: &[],
};
