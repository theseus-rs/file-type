use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866506: FileFormat = FileFormat {
    id: 105_866_506,
    puid: "wikidata/105866506",
    name: "X-CAD Pattern Fill",
    extensions: &["ptf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x50, 0x46, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
