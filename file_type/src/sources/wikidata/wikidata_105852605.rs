use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852605: FileFormat = FileFormat {
    id: 105_852_605,
    puid: "wikidata/105852605",
    name: "Rhinoceros STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x68, 0x69, 0x6E, 0x6F, 0x63, 0x65, 0x72, 0x6F, 0x73, 0x20, 0x42, 0x69,
                    0x6E, 0x61, 0x72, 0x79, 0x20, 0x53, 0x54, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
