use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866246: FileFormat = FileFormat {
    id: 105_866_246,
    source_type: SourceType::Wikidata,
    name: "PhotoList",
    extensions: &["pl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4C, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
