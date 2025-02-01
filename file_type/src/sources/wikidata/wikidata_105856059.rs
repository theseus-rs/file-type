use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856059: FileFormat = FileFormat {
    id: 105_856_059,
    puid: "wikidata/105856059",
    name: "FL Studio DrumKit (v2)",
    extensions: &["dmkit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x53, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
