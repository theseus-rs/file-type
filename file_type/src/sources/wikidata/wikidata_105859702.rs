use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859702: FileFormat = FileFormat {
    id: 105_859_702,
    puid: "wikidata/105859702",
    name: "Vioso warp/blend data",
    extensions: &["vwf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x77, 0x66, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
