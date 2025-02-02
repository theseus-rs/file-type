use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852734: FileFormat = FileFormat {
    id: 105_852_734,
    source_type: SourceType::Wikidata,
    name: "Session Description Protocol",
    extensions: &["sdp"],
    media_types: &["application/sdp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x3D, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
