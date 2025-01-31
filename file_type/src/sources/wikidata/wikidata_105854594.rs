use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854594: FileFormat = FileFormat {
    id: 105_854_594,
    puid: "wikidata/105854594",
    name: "Adorage Animation",
    extensions: &["ado"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x0D, 0x46, 0x45, 0x45, 0x5F, 0x41, 0x6E, 0x69, 0x6D, 0x5F, 0x56, 0x31,
                    0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
