use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861226: FileFormat = FileFormat {
    id: 105_861_226,
    puid: "wikidata/105861226",
    name: "interLaced eXtensible Trace",
    extensions: &["lxt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
