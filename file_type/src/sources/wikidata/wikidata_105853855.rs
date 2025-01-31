use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853855: FileFormat = FileFormat {
    id: 105_853_855,
    puid: "wikidata/105853855",
    name: "askSam Windows database",
    extensions: &["ask"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x73, 0x6B, 0x77])],
            },
        }],
    }],
    related_formats: &[],
};
