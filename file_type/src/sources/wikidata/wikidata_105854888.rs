use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854888: FileFormat = FileFormat {
    id: 105_854_888,
    source_type: SourceType::Wikidata,
    name: "Alpha Four record Set",
    extensions: &["set"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x09, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
