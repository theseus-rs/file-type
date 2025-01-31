use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854114: FileFormat = FileFormat {
    id: 105_854_114,
    puid: "wikidata/105854114",
    name: "1Password Cloud Keychain",
    extensions: &["attachment"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x50, 0x43, 0x4C, 0x44, 0x41, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
