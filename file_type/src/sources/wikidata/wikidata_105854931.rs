use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854931: FileFormat = FileFormat {
    id: 105_854_931,
    source_type: SourceType::Wikidata,
    name: "mkwACT lossless compressed audio (v1)",
    extensions: &["mkw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x6B, 0x77, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
