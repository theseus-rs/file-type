use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853370: FileFormat = FileFormat {
    id: 105_853_370,
    puid: "wikidata/105853370",
    name: "STuetzerbach Format",
    extensions: &["stf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x46, 0x31, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
