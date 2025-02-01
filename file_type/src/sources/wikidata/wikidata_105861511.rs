use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861511: FileFormat = FileFormat {
    id: 105_861_511,
    puid: "wikidata/105861511",
    name: "LMMC encoded router config",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4D, 0x4D, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
