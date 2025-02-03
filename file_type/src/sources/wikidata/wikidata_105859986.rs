use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859986: FileFormat = FileFormat {
    id: 105_859_986,
    source_type: SourceType::Wikidata,
    name: "XBV video",
    extensions: &["xbv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x46, 0x4D, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
