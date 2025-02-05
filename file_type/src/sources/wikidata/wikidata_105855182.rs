use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855182: FileFormat = FileFormat {
    id: 105_855_182,
    source_type: SourceType::Wikidata,
    name: "FireDAC database",
    extensions: &["fds"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x42, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
