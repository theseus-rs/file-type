use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851196: FileFormat = FileFormat {
    id: 105_851_196,
    puid: "wikidata/105851196",
    name: "Turboprint driver (v4)",
    extensions: &["tpp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x75, 0x72, 0x62, 0x6F, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x20, 0x34,
                    0x2E, 0x30, 0x20, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x5D, 0x0A, 0x44, 0x72,
                    0x69, 0x76, 0x65, 0x72, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
