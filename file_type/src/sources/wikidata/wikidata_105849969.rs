use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849969: FileFormat = FileFormat {
    id: 105_849_969,
    puid: "wikidata/105849969",
    name: "Rad Cad drawing",
    extensions: &["cad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x02, 0xDF, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
