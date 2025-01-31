use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854234: FileFormat = FileFormat {
    id: 105_854_234,
    puid: "wikidata/105854234",
    name: "WSL compressed data",
    extensions: &["wsl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x53, 0x4C, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
