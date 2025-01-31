use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855218: FileFormat = FileFormat {
    id: 105_855_218,
    puid: "wikidata/105855218",
    name: "ZODB File Storage (2.1)",
    extensions: &["fs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x32, 0x31, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
