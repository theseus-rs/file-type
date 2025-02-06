use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860879: FileFormat = FileFormat {
    id: 105_860_879,
    source_type: SourceType::Wikidata,
    name: "RAGE Package Format (Table Tennis)",
    extensions: &["rpf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x46, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
