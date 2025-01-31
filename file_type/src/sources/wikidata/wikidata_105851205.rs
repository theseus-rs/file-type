use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851205: FileFormat = FileFormat {
    id: 105_851_205,
    puid: "wikidata/105851205",
    name: "Xoom Tutor tutorial",
    extensions: &["tut"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x20, 0x69, 0x6E, 0x6D, 0x65, 0x6E, 0x75,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
