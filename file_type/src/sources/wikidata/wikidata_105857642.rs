use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857642: FileFormat = FileFormat {
    id: 105_857_642,
    source_type: SourceType::Wikidata,
    name: "Grand Theft Auto: San Andreas game data archive",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x45, 0x52, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
