use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857576: FileFormat = FileFormat {
    id: 105_857_576,
    puid: "wikidata/105857576",
    name: "InstallShield Script",
    extensions: &["ins"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB8, 0xC9, 0x0C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
