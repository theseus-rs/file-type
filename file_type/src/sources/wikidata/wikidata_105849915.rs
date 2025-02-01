use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849915: FileFormat = FileFormat {
    id: 105_849_915,
    puid: "wikidata/105849915",
    name: "Calamus Raster Information",
    extensions: &["cri"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20, 0x46,
                    0x47, 0x45, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
