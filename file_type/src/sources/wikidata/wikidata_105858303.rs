use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858303: FileFormat = FileFormat {
    id: 105_858_303,
    source_type: SourceType::Wikidata,
    name: "Total War campaign settings (var 1)",
    extensions: &["esf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
