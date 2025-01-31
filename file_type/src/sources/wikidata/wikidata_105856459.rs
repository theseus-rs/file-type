use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856459: FileFormat = FileFormat {
    id: 105_856_459,
    puid: "wikidata/105856459",
    name: "Wintrac log data (v3)",
    extensions: &["wtf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x74, 0x72, 0x61, 0x63, 0x20, 0x33, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
