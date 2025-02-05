use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860932: FileFormat = FileFormat {
    id: 105_860_932,
    source_type: SourceType::Wikidata,
    name: "Alpha Four field rules",
    extensions: &["rln"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x08, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
