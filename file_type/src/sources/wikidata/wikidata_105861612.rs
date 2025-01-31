use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861612: FileFormat = FileFormat {
    id: 105_861_612,
    puid: "wikidata/105861612",
    name: "CWLS Log ASCII Standard",
    extensions: &["las"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7E, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
