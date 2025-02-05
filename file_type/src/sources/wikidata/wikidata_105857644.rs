use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857644: FileFormat = FileFormat {
    id: 105_857_644,
    source_type: SourceType::Wikidata,
    name: "ZEMU IO Map",
    extensions: &["iom"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
