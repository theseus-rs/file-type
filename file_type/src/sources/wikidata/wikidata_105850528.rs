use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850528: FileFormat = FileFormat {
    id: 105_850_528,
    source_type: SourceType::Wikidata,
    name: "Cricket Audio Bank",
    extensions: &["ckb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x6B, 0x6D, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
