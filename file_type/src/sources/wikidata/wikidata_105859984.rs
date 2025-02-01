use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859984: FileFormat = FileFormat {
    id: 105_859_984,
    puid: "wikidata/105859984",
    name: "VZ200/300 image (type F1)",
    extensions: &["vz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x20, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
