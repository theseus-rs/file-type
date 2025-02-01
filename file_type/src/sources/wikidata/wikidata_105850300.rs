use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850300: FileFormat = FileFormat {
    id: 105_850_300,
    puid: "wikidata/105850300",
    name: "Calamus Font Data",
    extensions: &["cfn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x43, 0x46, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
