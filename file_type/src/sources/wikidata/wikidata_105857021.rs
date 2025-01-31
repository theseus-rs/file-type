use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857021: FileFormat = FileFormat {
    id: 105_857_021,
    puid: "wikidata/105857021",
    name: "MegaZeux General Digital Music",
    extensions: &["gdm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x4D, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
