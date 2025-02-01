use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851548: FileFormat = FileFormat {
    id: 105_851_548,
    puid: "wikidata/105851548",
    name: "THE SPREADSHEET worksheet",
    extensions: &["ts1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x52, 0x44, 0x44, 0x41, 0x54, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
