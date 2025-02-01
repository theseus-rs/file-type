use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854032: FileFormat = FileFormat {
    id: 105_854_032,
    puid: "wikidata/105854032",
    name: "WordPerfect Printer info",
    extensions: &["all"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6A, 0x62, 0x68, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
