use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854364: FileFormat = FileFormat {
    id: 105_854_364,
    source_type: SourceType::Wikidata,
    name: "SimAnt saved game (Amiga)",
    extensions: &["ant"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x47, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
