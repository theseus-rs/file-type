use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856620: FileFormat = FileFormat {
    id: 105_856_620,
    puid: "wikidata/105856620",
    name: "WinPlot data (v3)",
    extensions: &["wp3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC1, 0x03, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
