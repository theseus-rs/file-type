use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853058: FileFormat = FileFormat {
    id: 105_853_058,
    puid: "wikidata/105853058",
    name: "Playmation Surface",
    extensions: &["sur"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x55, 0x52, 0x46, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
