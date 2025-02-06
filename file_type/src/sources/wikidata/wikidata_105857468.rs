use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857468: FileFormat = FileFormat {
    id: 105_857_468,
    source_type: SourceType::Wikidata,
    name: "5View capture",
    extensions: &["5vw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0xAA, 0xAA, 0xAA])],
            },
        }],
    }],
    related_formats: &[],
};
