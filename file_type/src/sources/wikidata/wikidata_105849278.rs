use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849278: FileFormat = FileFormat {
    id: 105_849_278,
    puid: "wikidata/105849278",
    name: "ST-Sound YM chiptune (generic)",
    extensions: &["ym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
