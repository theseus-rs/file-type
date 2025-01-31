use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851141: FileFormat = FileFormat {
    id: 105_851_141,
    puid: "wikidata/105851141",
    name: "TransCopy disk image",
    extensions: &["tc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0xA5])],
            },
        }],
    }],
    related_formats: &[],
};
