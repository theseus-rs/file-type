use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861273: FileFormat = FileFormat {
    id: 105_861_273,
    puid: "wikidata/105861273",
    name: "Leonard Guides compiled data",
    extensions: &["lg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x47, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
