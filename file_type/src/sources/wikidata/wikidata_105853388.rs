use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853388: FileFormat = FileFormat {
    id: 105_853_388,
    puid: "wikidata/105853388",
    name: "SOSI map data",
    extensions: &["sos"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x48, 0x4F, 0x44, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
