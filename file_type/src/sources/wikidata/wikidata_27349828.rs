use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349828: FileFormat = FileFormat {
    id: 27_349_828,
    puid: "wikidata/27349828",
    name: "ESRI Arc/Info ASCII Grid",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x63, 0x6F, 0x6C, 0x73, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
