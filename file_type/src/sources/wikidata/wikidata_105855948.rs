use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855948: FileFormat = FileFormat {
    id: 105_855_948,
    puid: "wikidata/105855948",
    name: "Disk Doubler compressed data",
    extensions: &["dd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAB, 0xCD, 0x00, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
