use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861550: FileFormat = FileFormat {
    id: 105_861_550,
    puid: "wikidata/105861550",
    name: "DNA Sequence Alignment",
    extensions: &["lav"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x3A, 0x6C, 0x61, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
