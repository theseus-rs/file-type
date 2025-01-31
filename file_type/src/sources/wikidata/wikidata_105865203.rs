use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865203: FileFormat = FileFormat {
    id: 105_865_203,
    puid: "wikidata/105865203",
    name: "InstallShield compiled setup Package",
    extensions: &["pkg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0xA3])],
            },
        }],
    }],
    related_formats: &[],
};
