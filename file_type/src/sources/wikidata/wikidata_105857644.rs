use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857644: FileFormat = FileFormat {
    id: 105_857_644,
    puid: "wikidata/105857644",
    name: "ZEMU IO Map",
    extensions: &["iom"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFC, 0x32, 0xDA, 0xE6])],
            },
        }],
    }],
    related_formats: &[],
};
