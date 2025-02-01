use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851149: FileFormat = FileFormat {
    id: 105_851_149,
    puid: "wikidata/105851149",
    name: "SDLTRS Configuration",
    extensions: &["t8c"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x63, 0x61, 0x6C, 0x65, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
