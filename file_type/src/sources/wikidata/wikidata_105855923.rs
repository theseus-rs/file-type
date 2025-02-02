use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855923: FileFormat = FileFormat {
    id: 105_855_923,
    source_type: SourceType::Wikidata,
    name: "XTreeGold graphics Driver",
    extensions: &["drv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x54, 0x47, 0x44, 0x52, 0x56, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
