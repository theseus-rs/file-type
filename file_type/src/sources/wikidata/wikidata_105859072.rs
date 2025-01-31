use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859072: FileFormat = FileFormat {
    id: 105_859_072,
    puid: "wikidata/105859072",
    name: "AVG update package",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x20, 0x41, 0x56, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
