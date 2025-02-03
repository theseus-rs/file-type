use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851890: FileFormat = FileFormat {
    id: 105_851_890,
    source_type: SourceType::Wikidata,
    name: "ShroomPlayer module",
    extensions: &["sho"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x68, 0x72, 0x6F])],
            },
        }],
    }],
    related_formats: &[],
};
