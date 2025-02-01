use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855288: FileFormat = FileFormat {
    id: 105_855_288,
    puid: "wikidata/105855288",
    name: "Face The Music Effect",
    extensions: &["ef"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
