use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855686: FileFormat = FileFormat {
    id: 105_855_686,
    puid: "wikidata/105855686",
    name: "Sequencer One song",
    extensions: &["one"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x6F, 0x6E, 0x67, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
