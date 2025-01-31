use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858962: FileFormat = FileFormat {
    id: 105_858_962,
    puid: "wikidata/105858962",
    name: "SkyRoads bitmap",
    extensions: &["lzs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4D, 0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
