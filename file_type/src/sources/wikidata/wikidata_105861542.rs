use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861542: FileFormat = FileFormat {
    id: 105_861_542,
    puid: "wikidata/105861542",
    name: "SimTex LBX game data container",
    extensions: &["lbx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAD, 0xFE, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
