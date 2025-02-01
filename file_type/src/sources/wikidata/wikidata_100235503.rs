use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100235503: FileFormat = FileFormat {
    id: 100_235_503,
    puid: "wikidata/100235503",
    name: "FinePrint file format",
    extensions: &["fp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
