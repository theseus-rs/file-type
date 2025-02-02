use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50604441: FileFormat = FileFormat {
    id: 50_604_441,
    source_type: SourceType::Wikidata,
    name: "SNAP Archive Data File",
    extensions: &["adf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x75, 0x72, 0x76, 0x65, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
