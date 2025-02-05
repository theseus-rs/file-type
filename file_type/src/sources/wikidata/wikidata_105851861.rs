use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851861: FileFormat = FileFormat {
    id: 105_851_861,
    source_type: SourceType::Wikidata,
    name: "Steem Engine",
    extensions: &["stp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x54, 0x65, 0x78, 0x74, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
