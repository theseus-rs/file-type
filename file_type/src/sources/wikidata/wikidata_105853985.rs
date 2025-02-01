use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853985: FileFormat = FileFormat {
    id: 105_853_985,
    puid: "wikidata/105853985",
    name: "MST Quintus Animation",
    extensions: &["ani"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x54, 0x2D, 0x51, 0x1A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
