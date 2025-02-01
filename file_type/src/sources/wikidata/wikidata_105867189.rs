use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867189: FileFormat = FileFormat {
    id: 105_867_189,
    puid: "wikidata/105867189",
    name: "mcrypt encrypted (v2.2)",
    extensions: &["nc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x6D, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
