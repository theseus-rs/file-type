use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860044: FileFormat = FileFormat {
    id: 105_860_044,
    puid: "wikidata/105860044",
    name: "IPLAY Enterprise Video",
    extensions: &["epv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
