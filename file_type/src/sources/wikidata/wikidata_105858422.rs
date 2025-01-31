use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858422: FileFormat = FileFormat {
    id: 105_858_422,
    puid: "wikidata/105858422",
    name: "Resident Evil player model data",
    extensions: &["emw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x00, 0xB0, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
