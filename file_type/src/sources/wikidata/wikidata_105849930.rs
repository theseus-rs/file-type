use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849930: FileFormat = FileFormat {
    id: 105_849_930,
    puid: "wikidata/105849930",
    name: "Chasys Draw IES drawing",
    extensions: &["cd5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5F, 0x43, 0x44, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
