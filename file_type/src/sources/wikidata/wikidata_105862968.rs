use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862968: FileFormat = FileFormat {
    id: 105_862_968,
    puid: "wikidata/105862968",
    name: "MOZART Percussion map",
    extensions: &["mzp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x45, 0x52, 0x43, 0x55, 0x53, 0x53, 0x49, 0x4F, 0x4E, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
