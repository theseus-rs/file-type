use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856032: FileFormat = FileFormat {
    id: 105_856_032,
    source_type: SourceType::Wikidata,
    name: "DemoShield Demo (v5.x)",
    extensions: &["dbd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0xF3, 0x4A, 0xF2, 0x4D, 0xF3, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
