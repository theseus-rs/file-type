use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861669: FileFormat = FileFormat {
    id: 105_861_669,
    puid: "wikidata/105861669",
    name: "RPG Maker 2000/2003 Map",
    extensions: &["lmu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x4C, 0x63, 0x66, 0x4D, 0x61, 0x70, 0x55, 0x6E, 0x69, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
