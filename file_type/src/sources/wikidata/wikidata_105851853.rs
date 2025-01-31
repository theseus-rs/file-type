use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851853: FileFormat = FileFormat {
    id: 105_851_853,
    puid: "wikidata/105851853",
    name: "Alibre Design STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x6C, 0x69, 0x62, 0x72, 0x65, 0x20, 0x73, 0x74, 0x6C, 0x20, 0x62, 0x69,
                    0x6E, 0x61, 0x72, 0x79, 0x20, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
