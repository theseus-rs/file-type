use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855973: FileFormat = FileFormat {
    id: 105_855_973,
    source_type: SourceType::Wikidata,
    name: "MindReader Dictionary (v2.x)",
    extensions: &["dic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x52, 0x44, 0x49, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
