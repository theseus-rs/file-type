use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851550: FileFormat = FileFormat {
    id: 105_851_550,
    puid: "wikidata/105851550",
    name: "Tempus Word NG Document (generic)",
    extensions: &["twd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x57, 0x44, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
