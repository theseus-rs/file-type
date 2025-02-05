use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853144: FileFormat = FileFormat {
    id: 105_853_144,
    source_type: SourceType::Wikidata,
    name: "NTRQ module",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x54, 0x52, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
