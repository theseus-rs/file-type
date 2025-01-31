use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861266: FileFormat = FileFormat {
    id: 105_861_266,
    puid: "wikidata/105861266",
    name: "Windows LM data",
    extensions: &["lm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x42, 0x4C, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
