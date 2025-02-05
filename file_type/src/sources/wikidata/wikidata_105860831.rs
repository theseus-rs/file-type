use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860831: FileFormat = FileFormat {
    id: 105_860_831,
    source_type: SourceType::Wikidata,
    name: "Rob Northen Compression (type 2)",
    extensions: &["rnc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4E, 0x43, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
