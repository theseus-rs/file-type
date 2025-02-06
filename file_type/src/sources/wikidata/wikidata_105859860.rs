use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859860: FileFormat = FileFormat {
    id: 105_859_860,
    source_type: SourceType::Wikidata,
    name: "8088 Corruption TMV video",
    extensions: &["tmv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4D, 0x41, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
