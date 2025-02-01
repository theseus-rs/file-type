use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857144: FileFormat = FileFormat {
    id: 105_857_144,
    puid: "wikidata/105857144",
    name: "HoLa! Snapshot",
    extensions: &["hls"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4C, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
