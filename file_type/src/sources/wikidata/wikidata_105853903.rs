use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853903: FileFormat = FileFormat {
    id: 105_853_903,
    source_type: SourceType::Wikidata,
    name: "Ultra-fast LZ compressed data",
    extensions: &["ulz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4C, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
