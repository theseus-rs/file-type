use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856670: FileFormat = FileFormat {
    id: 105_856_670,
    source_type: SourceType::Wikidata,
    name: "Unity Player Preferences",
    extensions: &["upp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x6E, 0x69, 0x74, 0x79, 0x50, 0x72, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
