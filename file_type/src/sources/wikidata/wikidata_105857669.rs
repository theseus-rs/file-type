use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857669: FileFormat = FileFormat {
    id: 105_857_669,
    source_type: SourceType::Wikidata,
    name: "SuperJAM! Instrument",
    extensions: &["instrument"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4E, 0x53, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
