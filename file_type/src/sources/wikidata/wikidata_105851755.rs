use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851755: FileFormat = FileFormat {
    id: 105_851_755,
    puid: "wikidata/105851755",
    name: "Playmation sculpture/model",
    extensions: &["seg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x55, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
