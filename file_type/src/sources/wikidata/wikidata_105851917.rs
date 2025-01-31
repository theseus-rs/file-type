use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851917: FileFormat = FileFormat {
    id: 105_851_917,
    puid: "wikidata/105851917",
    name: "Sound Effect Editor format (v3)",
    extensions: &["see"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x45, 0x45, 0x33, 0x45, 0x44, 0x49, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
