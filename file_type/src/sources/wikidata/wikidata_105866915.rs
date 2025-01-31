use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866915: FileFormat = FileFormat {
    id: 105_866_915,
    puid: "wikidata/105866915",
    name: "NWiper show",
    extensions: &["nw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x57, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
