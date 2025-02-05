use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853231: FileFormat = FileFormat {
    id: 105_853_231,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Symbols (v2.x)",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x6D, 0x62, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
