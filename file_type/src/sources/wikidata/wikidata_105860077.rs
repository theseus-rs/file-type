use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860077: FileFormat = FileFormat {
    id: 105_860_077,
    source_type: SourceType::Wikidata,
    name: "CRYO HNM6 video",
    extensions: &["hnm", "hns"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4E, 0x4D, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
