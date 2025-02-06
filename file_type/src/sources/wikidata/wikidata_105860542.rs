use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860542: FileFormat = FileFormat {
    id: 105_860_542,
    source_type: SourceType::Wikidata,
    name: ".NET Framework Resource data",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBE, 0xEF, 0xCA, 0xCE])],
            },
        }],
    }],
    related_formats: &[],
};
