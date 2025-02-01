use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857115: FileFormat = FileFormat {
    id: 105_857_115,
    puid: "wikidata/105857115",
    name: "FL Studio GM Synth",
    extensions: &["gmsynth"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x6E, 0x79, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
