use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853937: FileFormat = FileFormat {
    id: 105_853_937,
    source_type: SourceType::Wikidata,
    name: "BSSC compressed data",
    extensions: &["bssc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x53, 0x53, 0x43, 0x5F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
