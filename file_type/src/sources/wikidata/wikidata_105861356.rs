use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861356: FileFormat = FileFormat {
    id: 105_861_356,
    puid: "wikidata/105861356",
    name: "lpaq1 compressed data",
    extensions: &["lpaq1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x51, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
