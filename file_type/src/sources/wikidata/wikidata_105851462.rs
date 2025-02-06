use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851462: FileFormat = FileFormat {
    id: 105_851_462,
    source_type: SourceType::Wikidata,
    name: "PSFTools human-readable textual font format",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x50, 0x53, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
