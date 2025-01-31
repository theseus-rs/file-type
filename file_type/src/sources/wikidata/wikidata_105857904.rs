use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857904: FileFormat = FileFormat {
    id: 105_857_904,
    puid: "wikidata/105857904",
    name: "UAE Input recorder data",
    extensions: &["inp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x41, 0x45, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
