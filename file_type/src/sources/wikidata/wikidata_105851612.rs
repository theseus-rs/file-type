use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851612: FileFormat = FileFormat {
    id: 105_851_612,
    puid: "wikidata/105851612",
    name: "PowerTablet Driver Template",
    extensions: &["template"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x4D, 0x50, 0x4C, 0x00, 0x00, 0x02, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
