use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865841: FileFormat = FileFormat {
    id: 105_865_841,
    puid: "wikidata/105865841",
    name: "Power Tab Guitar and Bass Tablature Editor",
    extensions: &["ptb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x74, 0x61, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
