use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853689: FileFormat = FileFormat {
    id: 105_853_689,
    puid: "wikidata/105853689",
    name: "LZBW1 compressed data",
    extensions: &["lz1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD2, 0xCC, 0x30, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
