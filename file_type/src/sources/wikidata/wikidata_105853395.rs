use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853395: FileFormat = FileFormat {
    id: 105_853_395,
    source_type: SourceType::Wikidata,
    name: "Superbase Program (var 3)",
    extensions: &["sbp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x50, 0x0A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
