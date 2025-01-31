use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861168: FileFormat = FileFormat {
    id: 105_861_168,
    puid: "wikidata/105861168",
    name: "Amiga WHDLoad package (lha compressed)",
    extensions: &["lha"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68, 0x35, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
