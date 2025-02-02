use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967131: FileFormat = FileFormat {
    id: 27_967_131,
    source_type: SourceType::Wikidata,
    name: "CyberTracker module",
    extensions: &["ct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
