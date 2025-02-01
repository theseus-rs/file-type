use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858219: FileFormat = FileFormat {
    id: 105_858_219,
    puid: "wikidata/105858219",
    name: "Express Publisher Document (DOS)",
    extensions: &["epd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x47, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
