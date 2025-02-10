use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856757: FileFormat = FileFormat {
    id: 105_856_757,
    source_type: SourceType::Wikidata,
    name: "Universal Scene Description (ASCII)",
    extensions: &["usd", "usda"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x75, 0x73, 0x64, 0x61, 0x20, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
