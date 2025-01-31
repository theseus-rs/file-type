use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855778: FileFormat = FileFormat {
    id: 105_855_778,
    puid: "wikidata/105855778",
    name: "Telepaint printer Driver",
    extensions: &["drv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x32, 0x01, 0x00, 0x4F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
