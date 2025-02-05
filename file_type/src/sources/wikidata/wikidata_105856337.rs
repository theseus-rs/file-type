use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856337: FileFormat = FileFormat {
    id: 105_856_337,
    source_type: SourceType::Wikidata,
    name: "DirectWave Program",
    extensions: &["dwp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x77, 0x50, 0x72])],
            },
        }],
    }],
    related_formats: &[],
};
