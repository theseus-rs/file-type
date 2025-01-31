use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856511: FileFormat = FileFormat {
    id: 105_856_511,
    puid: "wikidata/105856511",
    name: "WinPlot data (v2)",
    extensions: &["wp2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x03, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
