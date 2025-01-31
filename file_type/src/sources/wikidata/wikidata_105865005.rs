use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865005: FileFormat = FileFormat {
    id: 105_865_005,
    puid: "wikidata/105865005",
    name: "PicoQuant Unified TTTR",
    extensions: &["ptu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x51, 0x54, 0x54, 0x54, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
