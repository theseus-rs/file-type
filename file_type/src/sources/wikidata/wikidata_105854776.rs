use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854776: FileFormat = FileFormat {
    id: 105_854_776,
    source_type: SourceType::Wikidata,
    name: "CUPL ABS",
    extensions: &["abs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x5B, 0x05, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
