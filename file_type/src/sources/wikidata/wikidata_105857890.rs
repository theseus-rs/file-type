use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857890: FileFormat = FileFormat {
    id: 105_857_890,
    puid: "wikidata/105857890",
    name: "RC579 PICCOLINE disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x43, 0x37, 0x35, 0x39, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
