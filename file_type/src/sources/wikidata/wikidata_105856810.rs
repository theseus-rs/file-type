use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856810: FileFormat = FileFormat {
    id: 105_856_810,
    puid: "wikidata/105856810",
    name: "IBM Graphing Assistant Graph",
    extensions: &["gra"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x72, 0x61, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
