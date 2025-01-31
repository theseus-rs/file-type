use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858664: FileFormat = FileFormat {
    id: 105_858_664,
    puid: "wikidata/105858664",
    name: "Bitstream Compressed Outline font",
    extensions: &["bco"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x31, 0x8F, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
