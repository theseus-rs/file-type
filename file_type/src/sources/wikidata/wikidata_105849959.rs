use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849959: FileFormat = FileFormat {
    id: 105_849_959,
    puid: "wikidata/105849959",
    name: "Frostbite Catalog",
    extensions: &["cat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x79, 0x61, 0x6E, 0x4E, 0x79, 0x61, 0x6E, 0x4E, 0x79, 0x61, 0x6E, 0x4E,
                    0x79, 0x61, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
