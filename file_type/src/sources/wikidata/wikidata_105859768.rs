use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859768: FileFormat = FileFormat {
    id: 105_859_768,
    source_type: SourceType::Wikidata,
    name: "CRYO HNM4 video",
    extensions: &["hnm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4E, 0x4D, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
