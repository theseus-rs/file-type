use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850232: FileFormat = FileFormat {
    id: 105_850_232,
    puid: "wikidata/105850232",
    name: "Atari Cassette tape image",
    extensions: &["cas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x55, 0x4A, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
