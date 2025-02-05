use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851819: FileFormat = FileFormat {
    id: 105_851_819,
    source_type: SourceType::Wikidata,
    name: "Atari Works Wordprocessor document",
    extensions: &["stw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x30, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
