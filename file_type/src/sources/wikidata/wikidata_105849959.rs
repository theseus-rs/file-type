use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849959: FileFormat = FileFormat {
    id: 105_849_959,
    source_type: SourceType::Wikidata,
    name: "Frostbite Catalog",
    extensions: &["cat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
