use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862233: FileFormat = FileFormat {
    id: 105_862_233,
    source_type: SourceType::Wikidata,
    name: "MSX Protracker 1.0 module",
    extensions: &["pro"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x54, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
