use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855089: FileFormat = FileFormat {
    id: 105_855_089,
    puid: "wikidata/105855089",
    name: "League Of Legends bones based Animation",
    extensions: &["anm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x72, 0x33, 0x64, 0x32, 0x61, 0x6E, 0x6D, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
