use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853985: FileFormat = FileFormat {
    id: 105_853_985,
    source_type: SourceType::Wikidata,
    name: "MST Quintus Animation",
    extensions: &["ani"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x54, 0x2D, 0x51, 0x1A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
