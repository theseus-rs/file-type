use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858727: FileFormat = FileFormat {
    id: 105_858_727,
    puid: "wikidata/105858727",
    name: "SuperJAM! Band",
    extensions: &["band"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x4E, 0x44, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
