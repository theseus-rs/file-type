use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851959: FileFormat = FileFormat {
    id: 105_851_959,
    puid: "wikidata/105851959",
    name: "STW disk image",
    extensions: &["stw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x57, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
