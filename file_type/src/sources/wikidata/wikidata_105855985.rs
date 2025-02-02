use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855985: FileFormat = FileFormat {
    id: 105_855_985,
    source_type: SourceType::Wikidata,
    name: "Aegis Draw 2000 v2.x drawing",
    extensions: &["drawing"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x31, 0x30, 0x39, 0x31, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
