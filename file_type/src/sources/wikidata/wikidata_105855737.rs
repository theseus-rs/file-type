use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855737: FileFormat = FileFormat {
    id: 105_855_737,
    source_type: SourceType::Wikidata,
    name: "Rollei Digital Camera RAW picture",
    extensions: &["dcr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x53, 0x43, 0x2D, 0x49, 0x6D, 0x61, 0x67, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
