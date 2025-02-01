use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849936: FileFormat = FileFormat {
    id: 105_849_936,
    puid: "wikidata/105849936",
    name: "16bit DOS COM C0NtRiVER protected",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4E, 0x54, 0x58, 0xE8])],
            },
        }],
    }],
    related_formats: &[],
};
