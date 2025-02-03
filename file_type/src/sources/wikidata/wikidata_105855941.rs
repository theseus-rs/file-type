use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855941: FileFormat = FileFormat {
    id: 105_855_941,
    source_type: SourceType::Wikidata,
    name: "DarkWave Studio module",
    extensions: &["dwp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x50, 0x57, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
