use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856442: FileFormat = FileFormat {
    id: 105_856_442,
    source_type: SourceType::Wikidata,
    name: "Smart Software WorkSheet (v2.1)",
    extensions: &["ws"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x32, 0x2E, 0x31, 0x0D, 0x0A,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
