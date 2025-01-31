use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862036: FileFormat = FileFormat {
    id: 105_862_036,
    puid: "wikidata/105862036",
    name: "MegaZeux MZMX image",
    extensions: &["mzm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x4D, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
