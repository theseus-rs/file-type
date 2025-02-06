use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852908: FileFormat = FileFormat {
    id: 105_852_908,
    source_type: SourceType::Wikidata,
    name: "Slicks 'n' Slide track data",
    extensions: &["ss"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x53, 0x53, 0x7E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
