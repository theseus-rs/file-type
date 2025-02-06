use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867362: FileFormat = FileFormat {
    id: 105_867_362,
    source_type: SourceType::Wikidata,
    name: "Star Wars Jedi Knight: Jedi Academy bot Navigation/routes info",
    extensions: &["nav"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x35, 0x56, 0x4E, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
