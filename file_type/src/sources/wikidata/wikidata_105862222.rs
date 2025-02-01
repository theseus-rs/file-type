use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862222: FileFormat = FileFormat {
    id: 105_862_222,
    puid: "wikidata/105862222",
    name: "Magnetic Graphics (V2)",
    extensions: &["gfx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x50, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
