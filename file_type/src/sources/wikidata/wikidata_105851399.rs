use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851399: FileFormat = FileFormat {
    id: 105_851_399,
    source_type: SourceType::Wikidata,
    name: "Epic TFP format",
    extensions: &["tfp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x54, 0x46, 0x50, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
