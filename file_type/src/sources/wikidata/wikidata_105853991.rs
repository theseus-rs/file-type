use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853991: FileFormat = FileFormat {
    id: 105_853_991,
    source_type: SourceType::Wikidata,
    name: "MDCD compressed archive (v1.0)",
    extensions: &["cd", "md"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x6D, 0x64, 0x0A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
