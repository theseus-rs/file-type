use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850837: FileFormat = FileFormat {
    id: 105_850_837,
    source_type: SourceType::Wikidata,
    name: "FL Studio bass drum preset",
    extensions: &["kik"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x6B, 0x69, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
