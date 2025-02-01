use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853521: FileFormat = FileFormat {
    id: 105_853_521,
    puid: "wikidata/105853521",
    name: "Quasijarus Strong Compression compressed data",
    extensions: &["z"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0xA1])],
            },
        }],
    }],
    related_formats: &[],
};
