use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851132: FileFormat = FileFormat {
    id: 105_851_132,
    puid: "wikidata/105851132",
    name: "Tempus Word NG Document (v5)",
    extensions: &["twd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x57, 0x44, 0x50, 0x05])],
            },
        }],
    }],
    related_formats: &[],
};
