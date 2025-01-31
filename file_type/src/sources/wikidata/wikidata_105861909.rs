use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861909: FileFormat = FileFormat {
    id: 105_861_909,
    puid: "wikidata/105861909",
    name: "Magnetic Hint",
    extensions: &["hnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x48, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
