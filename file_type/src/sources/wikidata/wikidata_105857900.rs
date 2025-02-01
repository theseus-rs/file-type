use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857900: FileFormat = FileFormat {
    id: 105_857_900,
    puid: "wikidata/105857900",
    name: "InterFont font (v1.0)",
    extensions: &["if"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x74, 0x65, 0x72, 0x46, 0x6F, 0x6E, 0x74, 0x20, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
