use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853426: FileFormat = FileFormat {
    id: 105_853_426,
    puid: "wikidata/105853426",
    name: "Compress compressed data",
    extensions: &["z"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9D])],
            },
        }],
    }],
    related_formats: &[],
};
