use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851963: FileFormat = FileFormat {
    id: 105_851_963,
    source_type: SourceType::Wikidata,
    name: "Sequence/Show",
    extensions: &["seq"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x45, 0x51, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
