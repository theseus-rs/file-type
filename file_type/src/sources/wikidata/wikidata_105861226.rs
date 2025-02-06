use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861226: FileFormat = FileFormat {
    id: 105_861_226,
    source_type: SourceType::Wikidata,
    name: "interLaced eXtensible Trace",
    extensions: &["lxt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
