use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853432: FileFormat = FileFormat {
    id: 105_853_432,
    puid: "wikidata/105853432",
    name: "ZEsarUX snapshot",
    extensions: &["zx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x58, 0x00, 0xC0, 0x00, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
