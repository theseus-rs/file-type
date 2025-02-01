use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861298: FileFormat = FileFormat {
    id: 105_861_298,
    puid: "wikidata/105861298",
    name: "Lucas Film Data - Panel",
    extensions: &["lfd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x4E, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
