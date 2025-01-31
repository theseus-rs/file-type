use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857435: FileFormat = FileFormat {
    id: 105_857_435,
    puid: "wikidata/105857435",
    name: "Japanese Word Processor Kanji Font",
    extensions: &["f00"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x37, 0x3F, 0x4E, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
