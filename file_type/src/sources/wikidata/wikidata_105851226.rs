use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851226: FileFormat = FileFormat {
    id: 105_851_226,
    puid: "wikidata/105851226",
    name: "Turbo Pascal configuration",
    extensions: &["tp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x50, 0x61, 0x73, 0x63, 0x61, 0x6C, 0x20,
                    0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
