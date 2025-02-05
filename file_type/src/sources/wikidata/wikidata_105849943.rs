use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849943: FileFormat = FileFormat {
    id: 105_849_943,
    source_type: SourceType::Wikidata,
    name: "Lotus 123 configuration (V2.4J)",
    extensions: &["cnf"],
    media_types: &["application/vnd.lotus-1-2-3"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x05, 0x0C])],
            },
        }],
    }],
    related_formats: &[],
};
