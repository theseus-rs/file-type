use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860609: FileFormat = FileFormat {
    id: 105_860_609,
    source_type: SourceType::Wikidata,
    name: "Random Access Compression format",
    extensions: &["rac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0xC3, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
