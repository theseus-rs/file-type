use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857473: FileFormat = FileFormat {
    id: 105_857_473,
    puid: "wikidata/105857473",
    name: "1Password encrypted data",
    extensions: &["1pif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6F, 0x70, 0x64, 0x61, 0x74, 0x61, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
