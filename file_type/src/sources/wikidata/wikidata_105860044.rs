use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860044: FileFormat = FileFormat {
    id: 105_860_044,
    source_type: SourceType::Wikidata,
    name: "IPLAY Enterprise Video",
    extensions: &["epv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
