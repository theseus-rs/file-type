use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857666: FileFormat = FileFormat {
    id: 105_857_666,
    source_type: SourceType::Wikidata,
    name: "Install Maker Pro project",
    extensions: &["iip"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x00, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
