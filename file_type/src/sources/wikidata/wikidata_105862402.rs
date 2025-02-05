use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862402: FileFormat = FileFormat {
    id: 105_862_402,
    source_type: SourceType::Wikidata,
    name: "MDP (FireAlpaca) drawing",
    extensions: &["mdp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x64, 0x69, 0x70, 0x61, 0x63, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
