use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865923: FileFormat = FileFormat {
    id: 105_865_923,
    puid: "wikidata/105865923",
    name: "Aegis ProMotion Camera",
    extensions: &["pcam"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x50, 0x43, 0x31, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
