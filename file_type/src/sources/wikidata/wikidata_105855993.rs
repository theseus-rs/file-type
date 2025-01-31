use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855993: FileFormat = FileFormat {
    id: 105_855_993,
    puid: "wikidata/105855993",
    name: "iGO Digital Elevation Map",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x45, 0x4D, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
