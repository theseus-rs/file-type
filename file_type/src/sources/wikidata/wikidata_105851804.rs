use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851804: FileFormat = FileFormat {
    id: 105_851_804,
    puid: "wikidata/105851804",
    name: "Photono-Software Stealther Skin",
    extensions: &["skn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x9C, 0xEC, 0xBD])],
            },
        }],
    }],
    related_formats: &[],
};
