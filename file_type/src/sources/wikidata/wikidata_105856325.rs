use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856325: FileFormat = FileFormat {
    id: 105_856_325,
    source_type: SourceType::Wikidata,
    name: "DemoShield Demo",
    extensions: &["dbd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x4C, 0x75, 0x5A, 0x6B, 0x55, 0x74, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
