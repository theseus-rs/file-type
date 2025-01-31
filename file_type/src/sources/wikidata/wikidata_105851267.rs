use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851267: FileFormat = FileFormat {
    id: 105_851_267,
    puid: "wikidata/105851267",
    name: "TfID - Text File IDentifier definition (v0.x)",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x54, 0x66, 0x49, 0x44, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22, 0x30, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
