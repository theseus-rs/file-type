use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858405: FileFormat = FileFormat {
    id: 105_858_405,
    source_type: SourceType::Wikidata,
    name: "Total War campaign settings (var 2)",
    extensions: &["esf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xCE, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
