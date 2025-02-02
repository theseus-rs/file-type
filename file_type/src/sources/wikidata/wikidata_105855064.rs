use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855064: FileFormat = FileFormat {
    id: 105_855_064,
    source_type: SourceType::Wikidata,
    name: "BIS WSS PCM audio",
    extensions: &["wss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x53, 0x53, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
