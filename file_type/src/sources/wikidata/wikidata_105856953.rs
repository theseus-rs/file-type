use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856953: FileFormat = FileFormat {
    id: 105_856_953,
    puid: "wikidata/105856953",
    name: "Farming Simulator terrein data",
    extensions: &["grle"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x52, 0x4C, 0x45, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
