use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855071: FileFormat = FileFormat {
    id: 105_855_071,
    puid: "wikidata/105855071",
    name: "Amiga Money data (v2) (generic)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x4D, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
