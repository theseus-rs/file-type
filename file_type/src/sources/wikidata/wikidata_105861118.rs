use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861118: FileFormat = FileFormat {
    id: 105_861_118,
    puid: "wikidata/105861118",
    name: "Handy savestate (v2)",
    extensions: &["lss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x53, 0x53, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
