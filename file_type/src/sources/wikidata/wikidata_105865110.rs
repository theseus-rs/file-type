use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865110: FileFormat = FileFormat {
    id: 105_865_110,
    puid: "wikidata/105865110",
    name: "InnovMetric Software Polygon Model",
    extensions: &["pol"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4F, 0x4C, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
