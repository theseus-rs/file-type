use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855367: FileFormat = FileFormat {
    id: 105_855_367,
    puid: "wikidata/105855367",
    name: "Foto-Mosaic-Edda Data Base",
    extensions: &["fmedb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0x45, 0x44, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
