use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855279: FileFormat = FileFormat {
    id: 105_855_279,
    puid: "wikidata/105855279",
    name: "HMZK Font",
    extensions: &["ft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4D, 0x5A, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
