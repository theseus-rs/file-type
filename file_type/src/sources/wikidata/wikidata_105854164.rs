use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854164: FileFormat = FileFormat {
    id: 105_854_164,
    source_type: SourceType::Wikidata,
    name: "WeChat Audio message (SILK codec)",
    extensions: &["aud"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x02, 0x23, 0x21, 0x53, 0x49, 0x4C, 0x4B, 0x5F, 0x56, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
