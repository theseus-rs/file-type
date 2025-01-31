use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865318: FileFormat = FileFormat {
    id: 105_865_318,
    puid: "wikidata/105865318",
    name: "HP LaserJet Printer Cartridge Metric",
    extensions: &["pcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0x0C, 0x10, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
