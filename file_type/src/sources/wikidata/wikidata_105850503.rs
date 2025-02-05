use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850503: FileFormat = FileFormat {
    id: 105_850_503,
    source_type: SourceType::Wikidata,
    name: "Cyberboard Move",
    extensions: &["gmv"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4D, 0x4F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
